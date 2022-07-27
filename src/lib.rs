use std::{
  collections::BTreeMap,
  ffi::OsStr,
  fs::{create_dir_all, File},
  io::{BufRead, BufReader, BufWriter, LineWriter, Write},
  path::{Path, PathBuf},
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread::{sleep, spawn},
  time::Duration,
};

pub use const_str::replace;
use parking_lot::Mutex;
use rmw_str::Str;

type Map = Arc<Mutex<BTreeMap<Box<[u8]>, Box<[u8]>>>>;

#[derive(Debug)]
pub struct Config {
  map: Map,
  run: Arc<AtomicBool>,
  env: PathBuf,
  prefix: Box<str>,
}

const ENV: &str = "env";

fn line_key(line: &str) -> Option<(&str, usize)> {
  let t = line.trim_start();
  if !t.starts_with('#') {
    if let Some(pos) = t.find('=') {
      return Some((t[..pos].trim_end(), pos + 1));
    }
  }
  None
}

fn save(path: impl AsRef<Path>, map: &mut BTreeMap<Box<[u8]>, Box<[u8]>>) {
  if map.is_empty() {
    return;
  }

  let mut li = vec![];

  macro_rules! push {
    ($k:expr,$v:expr) => {
      li.push([&$k, &b" = "[..], &$v, &[b'\n'][..]].concat());
    };
  }

  if let Ok(env) = err::ok!(File::open(&path)) {
    for line in BufReader::new(env).lines().flatten() {
      if let Some((k, _)) = line_key(&line) {
        let k = k.as_bytes();
        if let Some(v) = map.get(k) {
          push!(k, v);
          map.remove(k);
          continue;
        }
      }
      let mut line = line.as_bytes().to_vec();
      line.push(b'\n');
      li.push(line);
    }
    for (k, v) in map.iter() {
      push!(k, v);
    }
  }

  *map = BTreeMap::new();

  if let Ok(file) = err::ok!(File::create(path)) {
    let mut w = LineWriter::new(BufWriter::new(file));
    for i in li {
      err::log!(w.write_all(&i));
    }
  }
}

impl Config {
  pub fn new(prefix: Box<str>) -> Self {
    let root = &env_dir::home(&prefix);
    let env = root.join(ENV);
    if err::ok!(create_dir_all(root)).is_ok() {
      if env.exists() {
        if let Ok(env) = err::ok!(File::open(&env)) {
          for line in BufReader::new(env).lines().flatten() {
            if let Some((key, pos)) = line_key(&line) {
              let key = format!("{}_{}", prefix, key);
              std::env::set_var(key, line[pos..].trim());
            }
          }
        }
      } else {
        err::log!(File::create(&env));
      }
    }

    Self {
      run: Arc::new(AtomicBool::new(false)),
      map: Map::default(),
      env,
      prefix,
    }
  }

  pub fn get<T: Str>(&self, key: impl AsRef<str>, init: impl Fn() -> T) -> T {
    let key_ref = key.as_ref();
    let key = format!("{}_{}", self.prefix, key_ref);

    self._get(&key, || {
      let r = init();
      let mut map = self.map.lock();

      let val = r.encode();
      std::env::set_var(&key, unsafe { &std::str::from_utf8_unchecked(&val) });
      map.insert(Box::from(key_ref.as_bytes()), val);

      if !self.run.fetch_or(true, Ordering::SeqCst) {
        let map = self.map.clone();
        let env = self.env.clone();
        let run = self.run.clone();
        spawn(move || {
          sleep(Duration::from_secs(1));
          save(env, &mut map.lock());
          run.store(false, Ordering::Relaxed);
        });
      }

      r
    })
  }

  fn _get<T: Str>(&self, key: impl AsRef<OsStr>, init: impl Fn() -> T) -> T {
    if let Ok(bin) = std::env::var(&key) {
      if let Ok(r) = err::ok!(T::decode(bin.as_bytes())) {
        return r;
      }
    }

    init()
  }
}

#[macro_export]
macro_rules! config {
  ($prefix:expr) => {
    let config = $crate::Config::new(stringify!(prefix).into());
    $crate::macro_def!(config, get);
  };
}

#[macro_export]
macro_rules! macro_def {
  ( $config:expr, $action:ident) => {
    macro_rules! $action {
      ($key:expr, $default:expr) => {
        $config.$action(
          $crate::replace!($crate::replace!(stringify!($key), " ", ""), "/", "_"),
          || $default,
        )
      };
    }
  };
}
