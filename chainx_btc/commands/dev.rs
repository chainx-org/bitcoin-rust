use clap::ArgMatches;
use sync::{create_sync_blocks_writer, Error};
use config::Config;
use util::init_db;

pub fn dev(cfg: Config) -> Result<(), String> {
	try!(init_db(&cfg));
	let mut writer = create_sync_blocks_writer(cfg.db, cfg.consensus, cfg.verification_params);

	let mut counter = 0;
	for blk in blk_dir {
		match writer.append_block(blk.block) {
			Ok(_) => {
				counter += 1;
				if counter % 1000 == 0 {
					info!(target: "sync", "Imported {} blocks", counter);
				}
			}
			Err(Error::TooManyOrphanBlocks) => return Err("Too many orphan (unordered) blocks".into()),
			Err(_) => return Err("Cannot append block".into()),
		}
	}

	info!("Finished import of {} blocks", counter);

	Ok(())
}
