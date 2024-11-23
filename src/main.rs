use std::fs::{read_dir, rename};

fn main() {
    let path = "./test";

    println!("Started execution on path {:?}", path);

    let mut v = Vec::new();
    for path in read_dir(path).unwrap() {
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

        if nmb != ct {
            let name = format!(
                "{}.{}",
                ct,
                elem.to_str().unwrap().split_once('.').unwrap().1
            );

            upd += 1;
            let _ = rename(
                path.to_owned() + "/" + elem.to_str().unwrap(),
                path.to_owned() + "/" + &name,
            );

            println!("\t{:?} --> {:?}", elem, name);
        }

        ct += 1;
    });

    print!("Finished execution - updated {} ", upd);
    println!("element{}", if upd != 1 { "s" } else { "" });
}
