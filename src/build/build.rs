use core::time;
use std::{fs, path::PathBuf};

use crate::build::valid;

pub fn build(f: PathBuf, d: &String, dest: &String) {
    let content = fs::read_to_string(&f).expect("error reading content");
    let mut aaa = clean(content) + "\n";
    if !valid::validate(&aaa) {
        return; //todo invalid
    }
    let e;
    let mut r = {
        let a = ..aaa.find(|c: char| c.is_whitespace() || c == '\n').unwrap();
        if aaa[a] == "".to_owned() {
            println!("no template given, skipping");
            return;
        }
        e = if aaa[a].find(".").is_some() {aaa.get(a).unwrap()[(&aaa[a]).rfind(".").unwrap() + 1..].to_string()} else {aaa[a].to_string()} ;
        let b = fs::read_to_string(d.to_owned() + "/" + &aaa[a]).unwrap_or("".to_owned());
        aaa.replace_range(a, "");
        b
    };
    println!("a{}a", aaa);
    println!("b{}b", r);
    if r == "" {
        println!("template not found, skipping");
        return;
    }
    let mut a: Option<usize>;
    a = aaa.find("$/");
    while a.is_some() {
        let c = aaa[a.unwrap() + 1..]
            .find(|i: char| i.is_whitespace() || i == '$')
            .map(|i| i + a.unwrap() + 1)
            .unwrap_or(aaa.len());
        println!("{}", &aaa[a.unwrap() + 2..c]);
        let b = fs::read_to_string(d.to_owned() + "/" + &aaa[a.unwrap() + 2..c])
            .expect("included file does not exist");
        aaa.replace_range(a.unwrap()..c, &clean(b));
        a = aaa.find("$/");
    }
    println!("c{}c", aaa);
    println!("d{}d", r);
    while aaa.contains("$$") {
        aaa = aaa.replace("$$", "$ $");
    }
    {
        let mut i = 0;
        for (ia, ib) in aaa.to_owned().chars().enumerate() {
            if ib == '$'
                && aaa[ia + i..].find(|i: char| i.is_whitespace()).unwrap_or(ia + i)
                    < aaa[ia + i..].find("(").unwrap_or(aaa.len() + i)
            {
                aaa = aaa[..ia + i].to_owned()
                    + &aaa[ia + i..]
                        .replacen(|i: char| i.is_whitespace(), "(", 1)
                        .replacen("\n", ")\n", 1);
                    i += 1;
            }
        }
    }
    aaa = aaa.replace("\n", "");
    a = aaa.find("  ");
    while a.is_some() {
        aaa = aaa.replace("  ", " ");
        a = aaa.find("  ");
    }
    r = r.replace("\n", "");
    a = r.find("  ");
    while a.is_some() {
        r = r.replace("  ", " "); //yes
        a = r.find("  ");
    }
    println!("e{}e", aaa);
    println!("f{}f", r);
    let mut rr = combine(&r, &aaa)
        .replace("&num;", "#")
        .replace("&dollar;", "$")
        .replace("&lpar;", "(")
        .replace("&rpar;", ")"); //add formatting to not have the entire html be one line
    while rr.contains("> ") {
        rr = rr.replace("> ", ">");
    }
    println!("g{}g", rr);
    println!("h{}h", e);
    fs::write(
        rl(&f
            .to_str()
            .expect("how did you even get here")
            .replacen(d, dest, 1))
            + &e,
        rr,
    )
    .expect("error writing file");
}

fn rl(a: &String) -> String {
    a[..a.len() - 1].to_owned()
}

fn combine(l: &String, r: &String) -> String {
    println!("0{}0{}0", l, r);
    let mut rr = r.to_owned();
    if l.find("$").is_none() {
        return l.to_owned();
    }
    if !rr.contains("$") {
        return l.replace("$", &rr);
    }
    let a = l.find("$").unwrap();
    let b = l.rfind(")").unwrap_or(a + 1) + 1;
    let lb = &l[..a];
    let lm = &l[a..b];
    let la = &l[b..];
    let mut f: String = "".to_owned();
    println!("1{}1{}1{}1", lb, lm, la);
    while rr.contains("$") {
        let (ra, rb, rc) = parse(&rr).expect("test");
        let (rh, rd) = (&rr.to_owned()[ra + 1..rb], &rr.to_owned()[rb + 1..rc]);
        let (mut lx, mut ly, mut lz) = parse(&lm.to_owned()).expect("test");
        let (mut lh, mut ld) = (&lm[lx + 1..ly], &lm[ly + 1..lz]);
        while lh != rh {
            println!("8{}8{}8{}8", lh, rh, lm);
            (lx, ly, lz) = parse(&lm[lz + 1..].to_owned())
                .map(|i| (i.0 + lz + 1, i.1 + lz + 1, i.2 + lz + 1))
                .expect("test");
            (lh, ld) = (&lm[lx + 1..ly], &lm[ly + 1..lz]);
        }
        println!("4{}4{}4", rh, rd);
        println!("5{}5{}5", lh, ld);
        rr = rr[rc + 1..].to_owned();
        f += &combine(&ld.to_owned(), &rd.to_owned());
    }
    lb.to_owned() + &f + la
}

fn parse(s: &String) -> Result<(usize, usize, usize), String> {
    println!("6{}6", s);
    if !s.contains("$") {
        return Err("not found".to_owned());
    }
    let mut i: usize = s.find("$").unwrap();
    let id: usize = i;
    let mut ib: usize = i + 1;
    let mut ic: usize = 0;
    let mut im: bool = false;
    while i < s.len() {
        if im {
            if &s[i..i + 1] == ")" {
                ic -= 1;
            }
            if &s[i..i + 1] == "(" {
                ic += 1;
            }
            if ic == 0 {
                println!("7{}7{}7{}7", id, ib, i);
                return Ok((id, ib, i));
            }
        } else {
            if &s[i..i + 1] == "(" {
                ib = i;
                im = true;
                ic = 1;
            }
        }
        i += 1;
    }
    Err("error".to_owned())
}

fn clean(content: String) -> String {
    let mut a: Option<usize>;
    let mut result = content.replace("\\#", "&num;");
    a = result.find("#");
    while a.is_some() {
        let c = {
            let b = result[a.unwrap()..].find("\n").map(|i| i + a.unwrap());
            if b.is_some() {
                b.unwrap()
            } else {
                result.len()
            }
        };
        result.replace_range(a.unwrap()..c + 1, "");
        a = result.find("#");
    }
    result
        .replace("\\$", "&dollar;")
        .replace("\\(", "&lpar;")
        .replace("\\)", "&rpar;")
}
