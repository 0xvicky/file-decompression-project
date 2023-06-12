use std::fs;
use std::io;


//There is a pattern in rust code, one function is main function and another function is used to cleanup the logic of main2 function
fn main(){
    std::process::exit(main2());//Cleanup the logic
}

fn main2() -> i32{
let args:Vec<_> = std::env::args().collect();//Here we're collecting the arguments

if args.len()<2 {// As we require two arguments, first is "Cargo run" and second is "zip     file name"
    println!("Usage: {} <filename>", args[0]); //In those curly braces this "arg[0] will be passed and then <filename>"
    return 1;
}

let fname = std::path::Path::new(&*args[1]);//The path to the filename
let file = fs::File::open(&fname).unwrap();//File will be read here
let mut archive = zip::ZipArchive::new(file).unwrap();// To process and to work with the file we, use this archive var

for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();//Go to every file one by one

    let outpath = match file.enclosed_name(){//To ensure safety or any attacker can overrwrite a critical path or malicious path to the fs
        Some(path)=>path.to_owned(),//cloning the path somewehere
        None=>continue,
    };
    {
        let comment  = file.comment();
        if !comment.is_empty(){
            println!("File {} comment:{}", i, comment);
        }
    }
    if(*file.name()).ends_with('/'){
        println!("File {} extracted to \"{}\"",i,outpath.display());
        fs::create_dir_all(&outpath).unwrap();
    }
    else{
        println!(
            "File {} extracted to \"{}\" ({} bytes)",//File extracted information will be here
            i,
            outpath.display(),
            file.size()
        );
        if let Some(p) = outpath.parent(){ //if file have parent then check for parent folders
            if !p.exists(){
                fs::create_dir_all(&p).unwrap();
            }
        }

        //creating outfile
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();
    
    };
    # [cfg(unix)]
    {
        use std::os::unix::fs::PermissionExt;

        if let Some(mode) = file.unix_mode(){
            fs::set_permissions(&outpath,fs::Permission::from_mode(mode)).unwrap();
        }
    }
}

0
}