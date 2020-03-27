use crate::common::*;
use create_step::CreateStep;

mod create_step;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Create a `.torrent` file.")
)]
pub(crate) struct Create {
  #[structopt(
    long = "announce",
    short = "a",
    value_name = "URL",
    help = "Use `URL` as the primary tracker announce URL. To supply multiple announce URLs, also \
            use `--announce-tier`."
  )]
  announce: Option<Url>,
  #[structopt(
    long = "allow",
    short = "A",
    value_name = "LINT",
    possible_values = Lint::VALUES,
    set(ArgSettings::CaseInsensitive),
    help = "Allow `LINT`. Lints check for conditions which, although permitted, are not usually \
            desirable. For example, piece length can be any non-zero value, but probably \
            shouldn't be below 16 KiB. The lint `small-piece-size` checks for this, and \
            `--allow small-piece-size` can be used to disable this check.",
  )]
  allowed_lints: Vec<Lint>,
  #[structopt(
    long = "announce-tier",
    short = "t",
    value_name = "URL-LIST",
    help = "Use `URL-LIST` as a tracker announce tier. Each instance adds a new \
            tier. To add multiple trackers to a given tier, separate their announce URLs \
            with commas:\n\
            \n\
            `--announce-tier udp://example.com:80/announce,https://example.net:443/announce`
            \n\
            Announce tiers are stored in the `announce-list` key of the top-level metainfo \
            dictionary as a list of lists of strings, as defined by BEP 12: Multitracker \
            Metadata Extension.
            \n\
            Note: Many BitTorrent clients do not implement the behavior described in BEP \
            12. See the discussion here for more details: \
            https://github.com/bittorrent/bittorrent.org/issues/82"
  )]
  announce_tiers: Vec<String>,
  #[structopt(
    long = "comment",
    short = "c",
    value_name = "TEXT",
    help = "Include `TEXT` as the comment for generated `.torrent` file. Stored under `comment` \
            key of top-level metainfo dictionary."
  )]
  comment: Option<String>,
  #[structopt(
    long = "node",
    value_name = "NODE",
    help = "Add DHT bootstrap node `NODE` to torrent. `NODE` should be in the form `HOST:PORT`, \
            where `HOST` is a domain name, an IPv4 address, or an IPv6 address surrounded by \
            brackets. May be given more than once to add multiple bootstrap nodes. Examples:
    `--node router.example.com:1337`
    `--node 203.0.113.0:2290`
    `--node [2001:db8:4275:7920:6269:7463:6f69:6e21]:8832`"
  )]
  dht_nodes: Vec<HostPort>,
  #[structopt(
    long = "dry-run",
    short = "n",
    help = "Skip writing `.torrent` file to disk."
  )]
  dry_run: bool,
  #[structopt(
    long = "follow-symlinks",
    short = "F",
    help = "Follow symlinks in torrent input. By default, symlinks to files and directories are \
            not included in torrent contents."
  )]
  follow_symlinks: bool,
  #[structopt(
    long = "force",
    short = "f",
    help = "Overwrite the destination `.torrent` file, if it exists."
  )]
  force: bool,
  #[structopt(
    long = "glob",
    short = "g",
    value_name = "GLOB",
    help = "Include or exclude files that match `GLOB`. Multiple glob may be provided, with the \
            last one taking precedence. Precede a glob with `!` to exclude it."
  )]
  globs: Vec<String>,
  #[structopt(
    long = "include-hidden",
    short = "h",
    help = "Include hidden files that would otherwise be skipped, such as files that start with a \
            `.`, and files hidden by file attributes on macOS and Windows."
  )]
  include_hidden: bool,
  #[structopt(
    long = "include-junk",
    short = "j",
    help = "Include junk files that would otherwise be skipped."
  )]
  include_junk: bool,
  #[structopt(
    long = "input",
    short = "i",
    value_name = "PATH",
    help = "Read torrent contents from `PATH`. If `PATH` is a file, torrent will be a single-file \
            torrent, if `PATH` is a directory, torrent will be a multi-file torrent.",
    parse(from_os_str)
  )]
  input: PathBuf,
  #[structopt(
    long = "link",
    help = "Print created torrent `magnet:` URL to standard output"
  )]
  print_magnet_link: bool,
  #[structopt(
    long = "md5",
    short = "M",
    help = "Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically \
            broken and only suitable for checking for accidental corruption."
  )]
  md5sum: bool,
  #[structopt(
    long = "name",
    short = "N",
    value_name = "TEXT",
    help = "Set name of torrent to `TEXT`. Defaults to the filename of the argument to `--input`."
  )]
  name: Option<String>,
  #[structopt(
    long = "no-created-by",
    help = "Do not populate `created by` key of generated torrent with imdl version information."
  )]
  no_created_by: bool,
  #[structopt(
    long = "no-creation-date",
    help = "Do not populate `creation date` key of generated torrent with current time."
  )]
  no_creation_date: bool,
  #[structopt(
    long = "open",
    short = "O",
    help = "Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on \
            Linux; `open` on macOS; and `cmd /C start` on Windows"
  )]
  open: bool,
  #[structopt(
    long = "order",
    value_name = "ORDER",
    possible_values = FileOrder::VALUES,
    set(ArgSettings::CaseInsensitive),
    help = "Specify the file order within the torrent. \
            Defaults to ascending alphabetical order."
  )]
  order: Option<FileOrder>,
  #[structopt(
    long = "output",
    short = "o",
    value_name = "TARGET",
    help = "Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is `-`. \
            Defaults to `$INPUT.torrent`.",
    parse(from_os_str)
  )]
  output: Option<OutputTarget>,
  #[structopt(
    long = "peer",
    value_name = "PEER",
    help = "Add `PEER` to magnet link.",
    requires("print-magnet-link")
  )]
  peers: Vec<HostPort>,
  #[structopt(
    long = "piece-length",
    short = "p",
    value_name = "BYTES",
    help = "Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib."
  )]
  piece_length: Option<Bytes>,
  #[structopt(
    long = "private",
    short = "P",
    help = "Set the `private` flag. Torrent clients that understand the flag and participate in \
            the swarm of a torrent with the flag set will only announce themselves to the \
            announce URLs included in the torrent, and will not use other peer discovery \
            mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for \
            more information."
  )]
  private: bool,
  #[structopt(
    long = "show",
    short = "S",
    help = "Display information about created torrent file."
  )]
  show: bool,
  #[structopt(
    long = "source",
    short = "s",
    value_name = "TEXT",
    help = "Set torrent source to `TEXT`. Stored under `source` key of info dictionary. This is \
            useful for keeping statistics from being mis-reported when participating in swarms \
            with the same contents, but with different trackers. When source is set to a unique \
            value for torrents with the same contents, torrent clients will treat them as \
            distinct torrents, and not share peers between them, and will correctly report \
            download and upload statistics to multiple trackers."
  )]
  source: Option<String>,
}

impl Create {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);

    let mut linter = Linter::new();
    linter.allow(self.allowed_lints.iter().cloned());

    let mut announce_list = Vec::new();
    for tier in &self.announce_tiers {
      let tier = tier.split(',').map(str::to_string).collect::<Vec<String>>();

      tier
        .iter()
        .map(|announce| announce.parse())
        .collect::<Result<Vec<Url>, url::ParseError>>()
        .context(error::AnnounceUrlParse)?;

      announce_list.push(tier);
    }

    if linter.is_denied(Lint::PrivateTrackerless) && self.private && self.announce.is_none() {
      return Err(Error::PrivateTrackerless);
    }

    CreateStep::Searching.print(env)?;

    let spinner = if env.err().is_styled_term() {
      let style = ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg:.bold}…")
        .tick_chars(consts::TICK_CHARS);

      Some(ProgressBar::new_spinner().with_style(style))
    } else {
      None
    };

    let files = Walker::new(&input)
      .include_junk(self.include_junk)
      .include_hidden(self.include_hidden)
      .follow_symlinks(self.follow_symlinks)
      .file_order(self.order.unwrap_or(FileOrder::AlphabeticalAsc))
      .globs(&self.globs)?
      .spinner(spinner)
      .files()?;

    let piece_length = self
      .piece_length
      .unwrap_or_else(|| PieceLengthPicker::from_content_size(files.total_size()));

    if piece_length.count() == 0 {
      return Err(Error::PieceLengthZero);
    }

    if linter.is_denied(Lint::UnevenPieceLength) && !piece_length.count().is_power_of_two() {
      return Err(Error::PieceLengthUneven {
        bytes: piece_length,
      });
    }

    if linter.is_denied(Lint::SmallPieceLength) && piece_length.count() < 16 * 1024 {
      return Err(Error::PieceLengthSmall);
    }

    let filename = input.file_name().ok_or_else(|| Error::FilenameExtract {
      path: input.clone(),
    })?;

    let name = match &self.name {
      Some(name) => name.clone(),
      None => filename
        .to_str()
        .ok_or_else(|| Error::FilenameDecode {
          filename: PathBuf::from(filename),
        })?
        .to_owned(),
    };

    let output = self
      .output
      .as_ref()
      .map(|output| output.resolve(env))
      .unwrap_or_else(|| {
        let mut torrent_name = name.to_owned();
        torrent_name.push_str(".torrent");

        OutputTarget::File(input.parent().unwrap().join(torrent_name))
      });

    if let OutputTarget::File(path) = &output {
      if !self.force && path.exists() {
        return Err(Error::OutputExists {
          path: path.to_owned(),
        });
      }
    }

    let private = if self.private { Some(true) } else { None };

    let creation_date = if self.no_creation_date {
      None
    } else {
      Some(
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)?
          .as_secs(),
      )
    };

    let created_by = if self.no_created_by {
      None
    } else {
      Some(String::from(consts::CREATED_BY_DEFAULT))
    };

    CreateStep::Hashing.print(env)?;

    let progress_bar = if env.err().is_styled_term() {
      let style = ProgressStyle::default_bar()
        .template(
          "{spinner:.green} ⟪{elapsed_precise}⟫ ⟦{bar:40.cyan}⟧ \
           {binary_bytes}/{binary_total_bytes} ⟨{binary_bytes_per_sec}, {eta}⟩",
        )
        .tick_chars(consts::TICK_CHARS)
        .progress_chars(consts::PROGRESS_CHARS);

      Some(ProgressBar::new(files.total_size().count()).with_style(style))
    } else {
      None
    };

    let (mode, pieces) = Hasher::hash(
      &files,
      self.md5sum,
      piece_length.as_piece_length()?.into_usize(),
      progress_bar,
    )?;

    CreateStep::Writing { output: &output }.print(env)?;

    let info = Info {
      source: self.source,
      piece_length,
      mode,
      pieces,
      name,
      private,
    };

    let metainfo = Metainfo {
      comment: self.comment,
      encoding: Some(consts::ENCODING_UTF8.to_string()),
      announce: self.announce.map(|url| url.to_string()),
      announce_list: if announce_list.is_empty() {
        None
      } else {
        Some(announce_list)
      },
      nodes: if self.dht_nodes.is_empty() {
        None
      } else {
        Some(self.dht_nodes)
      },
      creation_date,
      created_by,
      info,
    };

    let bytes = metainfo.serialize()?;

    if !self.dry_run {
      match &output {
        OutputTarget::File(path) => {
          let mut open_options = fs::OpenOptions::new();

          if self.force {
            open_options.write(true).create(true).truncate(true);
          } else {
            open_options.write(true).create_new(true);
          }

          open_options
            .open(path)
            .and_then(|mut file| file.write_all(&bytes))
            .context(error::Filesystem { path })?;
        }
        OutputTarget::Stdout => env.out_mut().write_all(&bytes).context(error::Stdout)?,
      }
    }

    #[cfg(test)]
    {
      let deserialized = bendy::serde::de::from_bytes::<Metainfo>(&bytes).unwrap();

      assert_eq!(deserialized, metainfo);

      let status = metainfo.verify(&input, None)?;

      if !status.good() {
        return Err(Error::Verify);
      }
    }

    errln!(env, "\u{2728}\u{2728} Done! \u{2728}\u{2728}")?;

    if self.show {
      TorrentSummary::from_metainfo(metainfo.clone())?.write(env)?;
    }

    if self.print_magnet_link {
      let mut link = MagnetLink::from_metainfo(&metainfo)?;
      for peer in self.peers {
        link.add_peer(peer);
      }
      outln!(env, "{}", link)?;
    }

    if let OutputTarget::File(path) = output {
      if self.open {
        Platform::open_file(&path)?;
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn require_input_argument() {
    let mut env = test_env! { args: [], tree: {} };
    assert!(matches!(env.run(), Err(Error::Clap { .. })));
  }

  #[test]
  fn require_input_present() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {},
    };
    assert!(matches!(env.run(), Err(Error::Filesystem { .. })));
  }

  #[test]
  fn announce_is_optional() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
  }

  #[test]
  fn torrent_file_is_bencode_dict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
      ],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let torrent = env.resolve("foo.torrent");
    let bytes = fs::read(torrent).unwrap();
    let value = Value::from_bencode(&bytes).unwrap();
    assert!(matches!(value, Value::Dict(_)));
  }

  #[test]
  fn input_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        ".",
        "--announce",
        "https://bar",
      ],
      cwd: "dir",
      tree: {
        dir: {
          foo: "",
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("../dir.torrent");
    assert_eq!(metainfo.info.name, "dir");
    assert_matches!(metainfo.info.mode, Mode::Multiple{files} if files.len() == 1);
  }

  #[test]
  fn input_dot_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "..",
        "--announce",
        "https://bar",
      ],
      cwd: "a/b",
      tree: {
        a: {
          b: {
            foo: "",
          },
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("../../a.torrent");
    assert_eq!(metainfo.info.name, "a");
    assert_matches!(metainfo.info.mode, Mode::Multiple{files} if files.len() == 1);
  }

  #[test]
  fn privacy_defaults_to_false() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "https://bar"],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.private, None);
  }

  #[test]
  fn privacy_flag_sets_privacy() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "https://bar", "--private"],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.private, Some(true));
  }

  #[test]
  fn tracker_flag_must_be_url() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "bar"],
      tree: {
        foo: "",
      }
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn announce_single() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "http://bar"],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce, Some("http://bar/".into()));
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_udp() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp://tracker.opentrackr.org:1337/announce",
      ],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.announce.as_deref(),
      Some("udp://tracker.opentrackr.org:1337/announce")
    );
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_wss_tracker() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "wss://tracker.btorrent.xyz",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.announce.as_deref(),
      Some("wss://tracker.btorrent.xyz/")
    );
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_single_tier() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--announce-tier",
        "http://bar,http://baz",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce.as_deref(), Some("http://bar/"));
    assert_eq!(
      metainfo.announce_list,
      Some(vec![vec!["http://bar".into(), "http://baz".into()]]),
    );
  }

  #[test]
  fn announce_multiple_tiers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--announce-tier",
        "http://bar,http://baz",
        "--announce-tier",
        "http://abc,http://xyz",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce.as_deref(), Some("http://bar/"));
    assert_eq!(
      metainfo.announce_list,
      Some(vec![
        vec!["http://bar".into(), "http://baz".into()],
        vec!["http://abc".into(), "http://xyz".into()],
      ])
    );
  }

  #[test]
  fn comment_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.comment, None);
  }

  #[test]
  fn comment_set() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--comment",
        "Hello, world!",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.comment.unwrap(), "Hello, world!");
  }

  #[test]
  fn piece_length_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes::from(16 * 2u32.pow(10)));
  }

  #[test]
  fn piece_length_override() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "64KiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes(64 * 1024));
  }

  #[test]
  fn si_piece_size() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "0.5MiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes(512 * 1024));
  }

  #[test]
  fn name() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "16KiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.name, "foo");
  }

  #[test]
  fn name_subdir() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo/bar",
        "--announce",
        "http://bar",
        "--piece-length",
        "32KiB",
      ],
      tree: {
        foo: {
          bar: "",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo/bar.torrent");
    assert_eq!(metainfo.info.name, "bar");
  }

  #[test]
  fn destination_override() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--output",
        "x.torrent",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    env.load_metainfo("x.torrent");
  }

  #[test]
  fn created_by_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.created_by.unwrap(), consts::CREATED_BY_DEFAULT);
  }

  #[test]
  fn created_by_unset() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-created-by",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.created_by, None);
  }

  #[test]
  fn encoding() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.encoding, Some("UTF-8".into()));
  }

  #[test]
  fn created_date_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert!(metainfo.creation_date.unwrap() < now + 10);
    assert!(metainfo.creation_date.unwrap() > now - 10);
  }

  #[test]
  fn created_date_unset() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.creation_date, None);
  }

  #[test]
  fn uneven_last_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "4",
        ],
      tree: {
        foo: "123",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["123"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    )
  }

  #[test]
  fn even_last_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "4",
        ],
      tree: {
        foo: "1234",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["1234"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(4),
        md5sum: None,
      }
    )
  }

  #[test]
  fn multi_piece_file() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "2",
        ],
      tree: {
        foo: "1234",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["12", "34"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(4),
        md5sum: None,
      }
    )
  }

  #[test]
  fn multi_file_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "dir",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "8",
        "--md5",
        ],
      tree: {
        dir: {
          foo: "1234",
          bar: "5678",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("dir.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["56781234"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Multiple {
        files: vec![
          FileInfo {
            path: FilePath::from_components(&["bar"]),
            length: Bytes(4),
            md5sum: Some(Md5Digest::from_data("5678")),
          },
          FileInfo {
            path: FilePath::from_components(&["foo"]),
            length: Bytes(4),
            md5sum: Some(Md5Digest::from_data("1234")),
          },
        ],
      }
    )
  }

  #[test]
  fn single_small() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        ],
      tree: {
        foo: "bar",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["bar"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    )
  }

  #[test]
  fn single_one_byte_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "1",
        "--allow",
        "small-piece-length",
      ],
      tree: {
        foo: "bar",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.info.pieces,
      PieceList::from_pieces(&["b", "a", "r"])
    );
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    )
  }

  #[test]
  fn single_empty() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces.count(), 0);
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(0),
        md5sum: None,
      }
    )
  }

  #[test]
  fn multiple_no_files() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {},
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces.count(), 0);
    assert_eq!(metainfo.info.mode, Mode::Multiple { files: Vec::new() })
  }

  #[test]
  fn multiple_one_file_md5() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {
        foo: {
          bar: "bar",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["bar"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[FileInfo {
            length: Bytes(3),
            md5sum: Some(Md5Digest::from_hex("37b51d194a7513e45b56f6524f2d51f2")),
            path: FilePath::from_components(&["bar"]),
          },]
        );
      }
      _ => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn multiple_one_file_md5_off() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          bar: "bar",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["bar"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[FileInfo {
            length: Bytes(3),
            md5sum: None,
            path: FilePath::from_components(&["bar"]),
          },]
        );
      }
      _ => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn multiple_three_files() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5"
      ],
      tree: {
        foo: {
          a: "abc",
          x: "xyz",
          h: "hij",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["abchijxyz"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("900150983cd24fb0d6963f7d28e17f72")),
              path: FilePath::from_components(&["a"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("857c4402ad934005eae4638a93812bf7")),
              path: FilePath::from_components(&["h"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("d16fb36f0911f878998c136191af705e")),
              path: FilePath::from_components(&["x"]),
            },
          ]
        );
      }
      _ => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn uneven_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "17KiB",
      ],
      tree: {
        foo: {},
      },
    };
    assert_matches!(
      env.run(),
      Err(Error::PieceLengthUneven { bytes }) if bytes.0 == 17 * 1024
    );
  }

  #[test]
  fn uneven_piece_length_allow() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "17KiB",
        "--allow",
        "uneven-piece-length",
      ],
      tree: {
        foo: {},
      },
    };
    env.run().unwrap();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn zero_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "0",
      ],
      tree: {
        foo: {},
      },
    };
    assert_matches!(env.run(), Err(Error::PieceLengthZero));
  }

  #[test]
  fn small_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "8KiB",
      ],
      tree: {
        foo: "",
      },
    };
    assert_matches!(env.run(), Err(Error::PieceLengthSmall));
  }

  #[test]
  fn small_piece_length_allow() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "8KiB",
        "--allow",
        "small-piece-length",
      ],
      tree: {
        foo: {},
      }
    };
    env.run().unwrap();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn output() {
    let mut env = TestEnvBuilder::new()
      .arg_slice(&[
        "imdl",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
      ])
      .out_is_term()
      .build();

    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a"), "abc").unwrap();
    fs::write(dir.join("x"), "xyz").unwrap();
    fs::write(dir.join("h"), "hij").unwrap();
    env.run().unwrap();
    assert_eq!(env.out(), "");
  }

  #[test]
  fn show() {
    let mut env = TestEnvBuilder::new()
      .arg_slice(&[
        "imdl",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
        "--show",
      ])
      .out_is_term()
      .build();

    let dir = env.resolve("foo");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a"), "abc").unwrap();
    fs::write(dir.join("x"), "xyz").unwrap();
    fs::write(dir.join("h"), "hij").unwrap();
    env.run().unwrap();
    let have = env.out();
    #[rustfmt::skip]
    let want = format!(
      "        Name  foo
  Created By  {}
   Info Hash  d3432a4b9d18baa413095a70f1e417021ceaca5b
Torrent Size  {} bytes
Content Size  9 bytes
     Private  no
     Tracker  http://bar/
  Piece Size  16 KiB
 Piece Count  1
  File Count  3
       Files  foo
              ├─a
              ├─h
              └─x
",
      consts::CREATED_BY_DEFAULT,
      212 + consts::CREATED_BY_DEFAULT.len()
    );
    assert_eq!(have, want);
  }

  #[test]
  fn write_to_stdout() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--output",
        "-",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let bytes = env.out_bytes();
    Metainfo::from_bytes(&bytes);
  }

  #[test]
  fn force_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar"
      ],
      tree: {
        foo: "",
        "foo.torrent": "foo",
      },
    };
    assert_matches!(
      env.run().unwrap_err(),
      Error::OutputExists {path}
      if path == env.resolve("foo.torrent")
    )
  }

  #[test]
  fn force_true() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--force",
      ],
      tree: {
        foo: "",
        "foo.torrent": "foo",
      },
    };
    env.run().unwrap();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn exclude_junk() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          "Thumbs.db": "abc",
          "Desktop.ini": "abc",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn include_junk() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--include-junk",
      ],
      tree: {
        foo: {
          "Thumbs.db": "abc",
          "Desktop.ini": "abc",
        },
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["abcabc"]));
  }

  #[test]
  fn skip_hidden() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          ".hidden": "abc",
          hidden: "abc",
        },
      }
    };

    if cfg!(target_os = "windows") {
      Command::new("attrib")
        .arg("+h")
        .arg(env.resolve("foo/hidden"))
        .status()
        .unwrap();
    } else if cfg!(target_os = "macos") {
      Command::new("chflags")
        .arg("hidden")
        .arg(env.resolve("foo/hidden"))
        .status()
        .unwrap();
    } else {
      fs::remove_file(env.resolve("foo/hidden")).unwrap();
    }

    env.run().unwrap();

    let metainfo = env.load_metainfo("foo.torrent");

    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 0
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn include_hidden() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--include-hidden",
      ],
      tree: {
        foo: {
          ".hidden": "abc",
          hidden: "abc",
        },
      }
    };

    if cfg!(target_os = "windows") {
      Command::new("attrib")
        .arg("+h")
        .arg(env.resolve("foo/hidden"))
        .status()
        .unwrap();
    } else if cfg!(target_os = "macos") {
      Command::new("chflags")
        .arg("hidden")
        .arg(env.resolve("foo/hidden"))
        .status()
        .unwrap();
    }

    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["abcabc"]));
  }

  fn populate_symlinks(env: &Env) {
    let dir = env.resolve("foo");
    let file_src = env.resolve("bar");
    let dir_src = env.resolve("dir-src");
    let dir_contents = dir_src.join("baz");
    fs::create_dir(&dir_src).unwrap();
    fs::write(dir_contents, "baz").unwrap();

    fs::create_dir(&dir).unwrap();
    fs::write(file_src, "bar").unwrap();
    #[cfg(unix)]
    {
      let file_link = env.resolve("foo/bar");
      let dir_link = env.resolve("foo/dir");
      Command::new("ln")
        .arg("-s")
        .arg("../bar")
        .arg(file_link)
        .status()
        .unwrap();

      Command::new("ln")
        .arg("-s")
        .arg("../dir-src")
        .arg(dir_link)
        .status()
        .unwrap();
    }
  }

  #[test]
  fn skip_symlinks() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {},
    };
    populate_symlinks(&env);
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  #[cfg(unix)]
  fn follow_symlinks() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--follow-symlinks",
        "--md5",
      ],
      tree: {},
    };
    populate_symlinks(&env);
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("barbaz").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("37b51d194a7513e45b56f6524f2d51f2")),
              path: FilePath::from_components(&["bar"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("73feffa4b7f6bb68e44cf984c85f6e88")),
              path: FilePath::from_components(&["dir", "baz"]),
            },
          ]
        );
      }
      _ => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  #[cfg(unix)]
  fn symlink_root() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {},
    };

    let file_src = env.resolve("bar");
    let file_link = env.resolve("foo");

    Command::new("ln")
      .arg("-s")
      .arg(&file_src)
      .arg(&file_link)
      .status()
      .unwrap();

    assert_matches!(env.run().unwrap_err(), Error::SymlinkRoot { root } if root == file_link);
  }

  #[test]
  fn skip_dot_dir_contents() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {
        foo: {
          ".bar": {
            baz: "baz",
          },
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn skip_hidden_attribute_dir_contents() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5"
      ],
      tree: {
        foo: {
          bar: {},
        },
      },
    };

    #[cfg(target_os = "windows")]
    {
      env.write("foo/bar/baz", "baz");
      let path = env.resolve("foo/bar");
      Command::new("attrib")
        .arg("+h")
        .arg(&path)
        .status()
        .unwrap();
    }

    #[cfg(target_os = "macos")]
    {
      env.write("foo/bar/baz", "baz");
      let path = env.resolve("foo/bar");
      Command::new("chflags")
        .arg("hidden")
        .arg(&path)
        .status()
        .unwrap();
    }

    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn glob_exclude() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!a"
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("bc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_exclude_nomatch() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!x"
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };

    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 3
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("abc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_include() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "[bc]",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("bc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_include_nomatch() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "x",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn glob_precedence() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!*",
        "--glob",
        "[ab]",
        "--glob",
        "!b",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(&["a"]));
  }

  #[test]
  fn nodes_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      }
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert!(metainfo.nodes.is_none());
  }

  #[test]
  fn nodes_invalid() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--node",
        "blah",
      ],
      tree: {
        foo: "",
      },
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn nodes_valid() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--node",
        "router.example.com:1337",
        "--node",
        "203.0.113.0:2290",
        "--node",
        "[2001:db8:4275:7920:6269:7463:6f69:6e21]:8832",
      ],
      tree: {
        foo: "",
      },
    };
    env.run().unwrap();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.nodes,
      Some(vec![
        "router.example.com:1337".parse().unwrap(),
        "203.0.113.0:2290".parse().unwrap(),
        "[2001:db8:4275:7920:6269:7463:6f69:6e21]:8832"
          .parse()
          .unwrap(),
      ]),
    );
  }

  #[test]
  fn create_progress_messages() {
    let mut env = TestEnvBuilder::new()
      .arg_slice(&[
        "imdl",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ])
      .build();

    fs::write(env.resolve("foo"), "").unwrap();

    let want = format!(
      "[1/3] \u{1F9FF} Searching for files…\n[2/3] \u{1F9EE} Hashing pieces…\n[3/3] \u{1F4BE} \
       Writing metainfo to `{}`…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n",
      env.resolve("foo.torrent").display()
    );

    env.run().unwrap();

    assert_eq!(env.err(), want);
  }

  #[test]
  fn private_requires_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--private",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(
      env.run(),
      Err(error @ Error::PrivateTrackerless)
      if error.lint() == Some(Lint::PrivateTrackerless)
    );
  }

  #[test]
  fn private_trackerless_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--private",
        "--allow",
        "private-trackerLESS",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
  }

  #[test]
  fn no_print_magnet_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
    assert_eq!(env.out(), "");
  }

  #[test]
  fn print_magnet_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--link",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
    assert_eq!(
      env.out(),
      "magnet:?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e&dn=foo\n"
    );
  }

  #[test]
  fn print_magnet_link_with_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--link",
        "--announce",
        "http://foo.com/announce",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
    assert_eq!(
      env.out(),
      "magnet:\
      ?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e\
      &dn=foo\
      &tr=http://foo.com/announce\n"
    );
  }

  #[test]
  fn peer_requires_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--peer",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn link_with_peers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--peer",
        "foo:1337",
        "--peer",
        "bar:666",
        "--link"
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Ok(()));
    assert_eq!(
      env.out(),
      "magnet:?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e&dn=foo&x.pe=foo:1337&x.pe=bar:\
       666\n"
    );
  }

  #[test]
  fn dry_run_skips_torrent_file_creation() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--dry-run",
      ],
      tree: {
        foo: "",
      }
    };
    assert_matches!(env.run(), Ok(()));
    let torrent = env.resolve("foo.torrent");
    let err = fs::read(torrent).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);
  }

  #[test]
  fn file_ordering_by_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "eeee",
          },
        },
      }
    };

    assert_matches!(env.run(), Ok(()));

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["a", "b", "c", "d/e"]);
  }

  #[test]
  fn file_ordering_by_alpha_asc() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--order",
        "alphabetical-asc",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "eeee",
          },
        },
      }
    };

    assert_matches!(env.run(), Ok(()));

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["a", "b", "c", "d/e"]);
  }

  #[test]
  fn file_ordering_by_alpha_desc() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--order",
        "alphabetical-desc",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            a: "aaaa",
          },
        },
      }
    };

    assert_matches!(env.run(), Ok(()));

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["d/a", "c", "b", "a"]);
  }

  #[test]
  fn file_ordering_by_size_asc() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--order",
        "size-asc",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "e",
          },
        },
      }
    };

    assert_matches!(env.run(), Ok(()));

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["b", "d/e", "a", "c"]);
  }

  #[test]
  fn file_ordering_by_size_desc() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--order",
        "size-desc",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "e",
          },
        },
      }
    };

    assert_matches!(env.run(), Ok(()));

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["c", "a", "b", "d/e"]);
  }
}