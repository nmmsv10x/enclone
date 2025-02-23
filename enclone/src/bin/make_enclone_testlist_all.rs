// Copyright (c) 2021 10X Genomics, Inc. All rights reserved.

// Make enclone/src/enclone.testlist.all.
//
// This records the id and cellranger version.

use enclone_core::defs::get_config;
use enclone_core::testlist::TEST_FILES_VERSION;
use io_utils::{dir_list, fwriteln, open_for_read, open_for_write_new, path_exists};
use pretty_trace::PrettyTrace;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use string_utils::TextUtils;

fn main() {
    PrettyTrace::new().on();
    let mut f = open_for_write_new!["enclone/src/enclone.testlist.all"];
    fwriteln!(
        f,
        "# List of all enclone test and development datasets.  These are largely"
    );
    fwriteln!(
        f,
        "# internal to 10x Genomics, for various reasons including because the data"
    );
    fwriteln!(
        f,
        "# are not in general consented for public release.  The purpose of this catalog"
    );
    fwriteln!(
        f,
        "# is to enable reconstruction of the datasets in the event of data loss."
    );
    fwriteln!(
        f,
        "# This file is autogenerated by enclone/src/bin/make_enclone_testlist_all.rs and should "
    );
    fwriteln!(f, "# not be manually edited.");
    fwriteln!(f, "");
    fwriteln!(f, "id,cellranger_version");
    let mut config = HashMap::<String, String>::new();
    let mut config_file = String::new();
    for (key, value) in env::vars() {
        if key == "ENCLONE_CONFIG" {
            config_file = value.to_string();
        }
    }
    let _ = get_config(&config_file, &mut config);
    let root = format!("{}/current{}", config["earth"], TEST_FILES_VERSION);
    let dirs = dir_list(&root);
    let mut ids = Vec::<usize>::new();
    for i in 0..dirs.len() {
        if dirs[i].parse::<usize>().is_ok() {
            let id = dirs[i].force_usize();
            ids.push(id);
        }
    }
    ids.sort_unstable();
    for i in 0..ids.len() {
        let id = ids[i];
        let mut version = "unknown".to_string();
        let log = format!("{}/{}/_log", root, id);
        if path_exists(&log) {
            let f = open_for_read![&log];
            for line in f.lines() {
                let s = line.unwrap();
                if s.contains(" [cmdline] ") && s.after(" [cmdline] ").contains(' ') {
                    let p = s.between(" [cmdline] ", " ");
                    if p.contains("cellranger-") {
                        let q = p.after("cellranger-");
                        if q.contains('/') {
                            version = format!("cellranger-{}", q.before("/").to_string());
                            if version.contains(".tar.gz") {
                                version = version.rev_before(".tar.gz").to_string();
                            }
                        }
                    }
                    break;
                }
            }
        }
        fwriteln!(f, "{},{}", id, version);
        println!("{} ==> {}", id, version);
    }
}
