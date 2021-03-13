use clap::Clap;
use tracing::debug;

use opt::{Opt, SubCommand};

use baby_git_rs::{
    cat_file, commit_tree, init_db, read_tree, show_diff, update_cache, write_tree, Config, Result,
};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    debug!(?config);

    let opt = Opt::parse();
    debug!(?opt);

    match opt.subcmd {
        SubCommand::InitDB => {
            init_db(&config)?;
            println!("defaulting to private storage area");
        }
        SubCommand::UpdateCache { paths } => update_cache(paths, &config)?,
        SubCommand::WriteTree => {
            let sha1 = write_tree(&config)?;
            println!("{}", sha1);
        }
        SubCommand::CommitTree { tree_hash, parents } => {
            if parents.is_empty() {
                println!("Committing initial tree {}", &tree_hash);
            }

            let sha1 = commit_tree(tree_hash, parents, &config)?;
            println!("{}", sha1);
        }
        SubCommand::ReadTree { tree_hash } => {
            let entries = read_tree(tree_hash, &config)?;
            for entry in entries {
                println!("{}", entry);
            }
        }
        SubCommand::ShowDiff => show_diff(&config)?,
        SubCommand::CatFile { object_hash } => {
            let (tmp_path, obj_type) = cat_file(object_hash, &config)?;
            println!("{:?}: {}", tmp_path, obj_type);
        }
    }

    Ok(())
}

mod opt {
    use std::path::PathBuf;

    use clap::Clap;

    use baby_git_rs::Sha1Hash;

    #[derive(Debug, Clap)]
    #[clap(author, about, version)]
    pub struct Opt {
        #[clap(subcommand)]
        pub(super) subcmd: SubCommand,
    }

    #[derive(Debug, Clap)]
    pub enum SubCommand {
        InitDB,
        UpdateCache {
            #[clap(required = true)]
            paths: Vec<PathBuf>,
        },
        WriteTree,
        CommitTree {
            #[clap(name("tree hash"), parse(try_from_str))]
            tree_hash: Sha1Hash,
            #[clap(short, name("parent commit hash"), parse(try_from_str))]
            parents: Vec<Sha1Hash>,
        },
        ReadTree {
            #[clap(name("tree hash"), parse(try_from_str))]
            tree_hash: Sha1Hash,
        },
        ShowDiff,
        CatFile {
            #[clap(name("object hash"), parse(try_from_str))]
            object_hash: Sha1Hash,
        },
    }
}
