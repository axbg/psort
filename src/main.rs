use std::fs::{read_dir, rename};

fn main() {
    let path = "./test";
    let paths = read_dir(path).unwrap();

    println!("Started execution on path {:?}", path);

    let mut v = Vec::new();
    for path in paths {
        let val = path.unwrap();

        if !val
            .file_name()
            .to_os_string()
            .into_string()
            .unwrap()
            .contains("DS_Store")
        {
            v.push(val.file_name().to_os_string());
        }
    }

    v.sort_by_key(|os_str| {
        os_str
            .to_str()
            .and_then(|s| s.split(".").next())
            .and_then(|prefix| prefix.parse::<i32>().ok())
            .unwrap_or(0)
    });

    let mut ct = 1;
    let mut upd = 0;

    v.iter().for_each(|elem| {
        let nmb = elem
            .to_str()
            .and_then(|s| s.split(".").next())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        let mut is_overwrite = false;
        if nmb != ct {
            is_overwrite = true;
        }

        let name = if is_overwrite {
            format!(
                "{}.{}",
                ct,
                elem.to_str().unwrap().split_once('.').unwrap().1
            )
        } else {
            elem.to_os_string().into_string().unwrap()
        };

        ct += 1;

        if is_overwrite {
            upd += 1;
            let _ = rename(
                path.to_owned() + "/" + elem.to_str().unwrap(),
                path.to_owned() + "/" + &name,
            );

            println!("\t{:?} --> {:?}", elem, name);
        }
    });

    print!("Finished execution - updated {} ", upd);
    println!("element{}", if upd != 1 { "s" } else { "" });
}
