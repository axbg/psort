use std::{
    fs::{read_dir, rename},
    i32,
};

fn main() {
    let pth = "/Users/axbg/Documents/Blogs/shadowed/public/pictures/thumbnails/";
    let paths =
        read_dir("/Users/axbg/Documents/Blogs/shadowed/public/pictures/thumbnails").unwrap();

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

    let mut nw = Vec::new();
    let mut ct = 1;
    let mut is_overwrite = true;

    v.iter().for_each(|elem| {
        let nmb = elem
            .to_str()
            .and_then(|s| s.split(".").next())
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        if nmb != ct {
            is_overwrite = true;
        }

        let name;
        if is_overwrite {
            name = ct.to_string()
                + "."
                + elem
                    .clone()
                    .into_string()
                    .unwrap()
                    .split_once(".")
                    .unwrap()
                    .1;
        } else {
            name = elem.clone().into_string().unwrap();
        }

        ct = ct + 1;
        nw.push(name);
    });

    for it in v.iter().zip(nw) {
        let (old, new) = it;
        rename(
            pth.to_owned() + old.to_str().unwrap(),
            pth.to_owned() + &new,
        );
    }
}
