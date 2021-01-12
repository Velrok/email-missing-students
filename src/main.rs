use serde::Deserialize;
use serde::Serialize;
use std::fs;

// libs needed
// - parse csv
// - parse args
// - send email

fn main() {
    // define args
    // read attended.csv
    // read class-list.csv
    // find missing studends
    // compile list of parents to email
    // generate email template
    // send idividual email after confirmation
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    let attendence: Vec<_> = read_attendence(opt.attendance.to_str().expect("exists"))
        .iter()
        .map(|r| r.email.to_owned())
        .collect();

    println!("--- In attendence ---");
    for a in &attendence {
        println!("{:?}", a);
    }

    let class = read_class_list(opt.class_list.to_str().expect("exists"));
    let missing: Vec<_> = class
        .iter()
        .filter(|r| !attendence.contains(&r.email))
        .collect();
    println!("--- missing ---");
    for a in &missing {
        println!("{:?}", a);
    }

    use tinytemplate::TinyTemplate;
    let mut tt = TinyTemplate::new();
    let template = fs::read_to_string(opt.email_template).expect("");
    tt.add_template("email", &template).unwrap();

    println!("--- emails ---");
    let emails: Vec<_> = missing
        .iter()
        .map(|r| tt.render("email", &r).unwrap())
        .collect();
    for e in emails {
        println!("{}", e);
        println!("------------");
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "email-missing-students")]
struct Opt {
    #[structopt(
        short = "c",
        long,
        parse(from_os_str),
        default_value = "./attendance.csv"
    )]
    attendance: PathBuf,

    #[structopt(
        short = "f",
        long,
        parse(from_os_str),
        default_value = "./class-list.csv"
    )]
    class_list: PathBuf,

    #[structopt(
        short = "e",
        long,
        parse(from_os_str),
        default_value = "./email-template.txt"
    )]
    email_template: PathBuf,
}

#[derive(Debug, Deserialize)]
struct AttendenceRecord {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClassRecord {
    name: String,
    email: String,
    parent_email: String,
}

fn read_attendence(file_name: &str) -> Vec<AttendenceRecord> {
    use std::fs::File;
    use std::io::BufReader;
    let mut rdr = csv::Reader::from_reader(BufReader::new(
        File::open(file_name).expect("file not found!"),
    ));
    rdr.deserialize()
        .map(|r| match r {
            Ok(r) => r,
            Err(e) => panic!(e.to_string()),
        })
        .collect()
}

fn read_class_list(file_name: &str) -> Vec<ClassRecord> {
    use std::fs::File;
    use std::io::BufReader;
    let mut rdr = csv::Reader::from_reader(BufReader::new(
        File::open(file_name).expect("file not found!"),
    ));
    rdr.deserialize()
        .map(|r| match r {
            Ok(r) => r,
            Err(e) => panic!(e.to_string()),
        })
        .collect()
}

#[derive(Serialize)]
struct Context {
    name: String,
}
