use std::collections::BTreeMap;

#[derive(Clone, Copy)]
struct FreeExtent(i64);

#[derive(Clone, Copy)]
struct FileExtent(i64, i64);

fn debug_print(file_set: &BTreeMap<i64, FileExtent>) {
    let mut last_off = 0;
    for (&off, &FileExtent(fid, len)) in file_set.iter() {
        print!("{}", ".".repeat((off - last_off) as usize));
        print!("{}", fid.to_string().repeat(len as usize));
        last_off = off + len;
    }
    println!();
}

fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();

    let mut free_set: BTreeMap<i64, FreeExtent> = BTreeMap::new();
    let mut file_set: BTreeMap<i64, FileExtent> = BTreeMap::new();

    let mut offset: i64 = 0;
    for (idx, n) in input.chars().enumerate() {
        let n = n.to_digit(10).unwrap() as i64;
        if n == 0 {
            continue;
        }
        match idx % 2 {
            0 => {
                let fid = idx as i64 / 2;
                file_set.insert(offset, FileExtent(fid, n));
            }
            _ => {
                free_set.insert(offset, FreeExtent(n));
            }
        }
        offset += n;
    }

    debug_print(&file_set);

    // part 1
    let ans1: i64 = {
        let mut free_set = free_set.clone();
        let mut moved_set: BTreeMap<i64, FileExtent> = BTreeMap::new();

        for (&file_off, &FileExtent(fid, mut file_len)) in file_set.iter().rev() {
            for (&free_off, &FreeExtent(free_len)) in free_set.clone().range(..file_off) {
                let moved_len = free_len.min(file_len);
                assert!(moved_set.insert(free_off, FileExtent(fid, moved_len)).is_none());
                free_set.remove(&free_off).unwrap();
                if free_len > moved_len {
                    assert!(free_set
                        .insert(free_off + moved_len, FreeExtent(free_len - moved_len))
                        .is_none());
                }
                file_len -= moved_len;
                if file_len == 0 {
                    break;
                }
            }

            if file_len > 0 {
                assert!(moved_set.insert(file_off, FileExtent(fid, file_len)).is_none());
            }
        }

        debug_print(&moved_set);

        moved_set
            .iter()
            .map(|(&off, &FileExtent(fid, len))| fid * len * (off * 2 + len - 1) / 2)
            .sum()
    };

    // part 2
    let ans2: i64 = {
        let mut free_set = free_set.clone();
        let mut moved_set: BTreeMap<i64, FileExtent> = BTreeMap::new();

        for (&file_off, &file_extent) in file_set.iter().rev() {
            let file_len = file_extent.1;
            match free_set.range(..file_off).find(|(_, &FreeExtent(len))| len >= file_len) {
                Some((&free_off, &FreeExtent(free_len))) => {
                    assert!(moved_set.insert(free_off, file_extent).is_none());
                    free_set.remove(&free_off).unwrap();
                    if free_len > file_len {
                        assert!(free_set
                            .insert(free_off + file_len, FreeExtent(free_len - file_len))
                            .is_none());
                    }
                }
                None => {
                    assert!(moved_set.insert(file_off, file_extent).is_none());
                }
            }
        }

        debug_print(&moved_set);

        moved_set
            .iter()
            .map(|(&off, &FileExtent(fid, len))| fid * len * (off * 2 + len - 1) / 2)
            .sum()
    };
    println!("{ans1},{ans2}");
}
