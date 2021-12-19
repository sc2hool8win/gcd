use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
//
use log::{trace,debug,info,warn};
//
fn main() {
    env::set_var("RUST_LOG", "warn");   // User Set:: log=> {trace,debug,info,warn};
    env_logger::init();
    trace!("#TRACE>DEBUG>INFO>WARN#");debug!("#DEBUG>INFO>WARN#");info!("#INFO>WARN#");
    // ///////////////////////////////////
    let args: Vec<String> = env::args().collect();
    trace!("LOG:My path is {}.", args[0]);
    trace!("LOG:I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    if git_chk() {
        let _top_dir_name: String = git_dir();
        trace!("git_dir: Top name is [{}]%", _top_dir_name);

        let search_dir;
        if args.len() > 1 {
            search_dir = &args[1];
        } else {
            search_dir = &_top_dir_name;
        }
        trace!("search_dir = [{}]%", search_dir );
        chk_cash_list(&_top_dir_name);

        let _path_gitcache = format!("{}{}", _top_dir_name, ".keepCache");
        let _path_head = format!("{}{}", _top_dir_name, ".git/logs/HEAD");
        let bool_auto_cache = new_head_time(&_path_gitcache, &_path_head);
        let _target = final_dir_target(&_top_dir_name, search_dir);
        info!("new_head_time = [{}]%", bool_auto_cache);
        info!("_target::{}",_target);
        if bool_auto_cache | ( search_dir == "//") | ( search_dir == "..")  {
            info!("create_keep_cache_list = TRUE");
            // making .keepCache
            create_keep_cache_list(&_top_dir_name.to_string());
        }
        // 
        let _gcd = get_gcd_path(&_top_dir_name, &_target);
        trace!("_gcd = [{}]", _gcd );
        print!("{}", _gcd);
    }
}

// ### repository check ###
fn git_chk() -> bool {
    let output = Command::new("git")
        .args(&["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("failed to start `git is-inside`");
    let out_val = String::from_utf8_lossy(&output.stdout);
    let _bool_val = match &*out_val.trim() { "true" => true, _ => false};
    debug!("git_chk bool is [{}].", _bool_val );
    return _bool_val
}
// ### repository path ###
fn git_dir() -> String{
    let output = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
        .expect("failed to start `git rev-parse`");
    trace!("DEBUG OUT1: {}", String::from_utf8_lossy(&output.stdout));
    let get_path = String::from_utf8_lossy(&output.stdout);
    let top_dir = get_path.trim() ;
    let ret_dir = top_dir.to_string()+ "/";
    return ret_dir
}

fn chk_cash_list( fn_top_dir:&str ) {
    let cache_file = fn_top_dir.to_string() + ".keepCache";
    // linux command "test -e .keepCache"
    let _output = Command::new("test")
        .args(&["-e",&cache_file])
        .status()
        .expect("get_cash_list test -e");
    if !(_output.success()) {
        // "touch .keepCache"
        let _output = Command::new("touch")
            .args(&[&cache_file])
            .status()
            .expect("chk_cash_list touch");
    }
}

fn new_head_time(_file1:&str, _file2:&str) -> bool{
    let output = Command::new("test")
        .args(&[_file1, "-ot",_file2])
        .status()
        .expect("Timestamp comparison`");
    let _comp_bool = output.success();
    trace!("_comp_bool: {}", &_comp_bool );
    return _comp_bool
}

fn final_dir_target(_top_dir:&str, _args1:&str) -> String{
    let pwd = env::current_dir().unwrap();
    let _pwd:String = pwd.display().to_string();
    trace!("pwd : {}", pwd.display() );
    trace!("_pwd: {}", &_pwd );

    //## BASE NAME ###
    let dir = Path::new(&_pwd).file_name().unwrap();
    let _dir = dir.to_str().unwrap();
    trace!("_dir: {}", _dir  );

    let mut _search_name:&str = _args1;
    /* ##### magic command ".." , add gitkeep ##### */
    if _args1 == ".." {
        // Linux command "touch .gitkeep"
        let _output = Command::new("touch")
            .args( &[".gitkeep"] )
            .status()
            .expect("final_dir_target touch gitkeep");
        _search_name = &_dir;
    }
    /* ##### magic command "//", Rebuilding ##### */
    if _args1 == "//" {
        _search_name = &_pwd;
    }
    debug!("final_dir_target::return _search_name: {}", _search_name );
    return _search_name.to_string()
}

fn create_keep_cache_list( _top_dir:&str){
    let _output = Command::new("find")
        .args(&[ _top_dir, "-name", ".gitkeep", "-type", "f"])
        .output()
        .expect("failed to start `find .gitkeep`");
    let _find_path = String::from_utf8_lossy(&_output.stdout);
    let _find_paths = _find_path.trim() ;
    debug!("create_keep_cache_list::_find_paths: {}", _find_paths );
    //### Make: LIST 1st
    let mut _vec_dlink:Vec<String> = Vec::new();
    let _v: Vec<&str> = _find_paths.split_whitespace().collect();
    for _dkeep in _v.iter() {
        if _dkeep.ends_with("/.gitkeep"){
            let _dkcashe= _dkeep.replace(".gitkeep","^");
            trace!("_dkcashe: {}", _dkcashe );
             _vec_dlink.push(_dkcashe);
        }
    }
    //### touch .keepSlink ###
    let _slink = _top_dir.to_string() + ".keepSlink";
    let _sread = _slink.clone();
    trace!("LOG:_slink:{}",_slink);
    let _dotk = Command::new("touch")
        .args(&[_slink])
        .status()
        .expect("touch keepSlink");
    //### LIST 2nd (Add user Synbolic link)
    let _slines = fs::read_to_string(_sread ).unwrap();
    let _vlines: Vec<&str> = _slines.split_whitespace().collect();
    //
    let mut _vec_slink:Vec<String> = Vec::new();
    for _line in _vlines.iter() {
        trace!("_line: {}", _line );
        if _line.starts_with("./") {
            let _abs_path =  format!("{}{}",_top_dir,_line);
            let _abs_path = _abs_path.replace("./","");
            if Path::new(&_abs_path).exists() {
                trace!("_abs_path: {}", _abs_path );
                let mut _end = "";
                if '/' != _abs_path.chars().last().unwrap(){
                    _end =  "/";
                }
                _vec_slink.push(_abs_path + _end + "^") ;
            }
        }
    }
    // joint list
    _vec_dlink.append(&mut _vec_slink);
    trace!("_vec_dlink: {}", _vec_dlink[0] );
    trace!("_vec_dlink: {}", _vec_dlink[2] );
    trace!("_vec_dlink: {}", _vec_dlink[4] );
    trace!("_vec_dlink: {}", _vec_dlink[5] );
    /* ### Mkae: LIST 3rd (Sort) ### */
    trace!("Vec Length {}",_vec_dlink.len());
    trace!("Vector: {:?}", _vec_dlink);
    let mut _vec_gcd:Vec<GcdPath> = Vec::new();

    '_for_vec: for _vec in _vec_dlink.iter() {
        trace!("_vec={}", _vec);
        let re = Regex::new(r"(.*/)(\w+/)\^").unwrap();
        //let caps = re.captures(_vec).unwrap();
        let caps = match re.captures(_vec) {
            Some(_x) => _x,
            None     => continue '_for_vec,
        };
        let capi:u32 = *(&caps[2].len()) as u32;
        trace!("caps2=>{}", &caps[2]);
        trace!("capi==>{:?}", capi);
        let caps12 = caps[1].to_string() + &caps[2];
        _vec_gcd.push(GcdPath::new( caps12 , capi)) ;
    }
    // ## struct GcdPath[(gcd_path,gcd_len)] sort by length ##
    _vec_gcd.sort_by(|a, b| a.gcd_len.cmp(&b.gcd_len));
    trace!("LOG:Vector: {:#?}", _vec_gcd);
    trace!("LOG:Vector: {:?}", _vec_gcd);
    trace!("LOG:{}",_vec_gcd[0].gcd_path);

    // ### Write List :: .keepCache ###
    let _write_fname = format!("{}{}", &_top_dir, ".keepCache");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&_write_fname).expect("Error: Open keepCache");
    let mut buf_writer = BufWriter::new(file);
    debug!("write_file={}", &_write_fname);
    for _p in _vec_gcd.iter() {
        debug!("W:{}",_p.gcd_path);
        writeln!(buf_writer, "{}",_p.gcd_path).expect("Error: Write keepCache");
    }
}

//
//
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct GcdPath {
    gcd_path: String,
    gcd_len: u32
}
impl GcdPath {
    pub fn new(gcd_path: String, gcd_len: u32) -> Self {
        GcdPath {
            gcd_path,
            gcd_len
        }
    }
}

fn get_gcd_path( _top_dir:&str, _search_dir:&str) -> String{
    let _cashe = _top_dir.to_string() + ".keepCache";
    let mut _return = _top_dir;
    let _cascl = _cashe.clone();
    let _spath = fs::read_to_string(_cashe).unwrap();
    let _vpath: Vec<&str> = _spath.split_whitespace().collect();
    //
    trace!("LOG:_spath: {}", _spath);
    trace!("LOG:_vpath: {:?}", _vpath);
    let _search_word = format!("/{}",_search_dir); // "/_search_dir"
    'outer: for _vp in _vpath.iter() {
        trace!("_vp:{:?}",_vp);
        //if _vp.ends_with("/examples/"){
        if _vp.find(&_search_word).is_some()    {
            debug!("mach!#247#:{},{}",_vp,_search_dir);
            debug!("mach!#248#:{},{}",_vp,_search_word);
            //
            let _vword: Vec<&str> = _vp.rsplit('/').collect(); // last word
            let _sword: Vec<&str> = _search_dir.rsplit('/').collect(); // last search
            debug!("#end word: {},{} {}",&_vword[1],_search_dir,&_sword[0]);
            //==== sense_vp: Concatenated path judgment ================
            let _nest = _search_dir.match_indices("/").count();
            debug!("_nest:{}",&_nest);
            let mut _nest_vp = _vword[1..=(_nest+1)].to_vec();
            _nest_vp.reverse();
            // join Directory Path
            let sense_vp = _nest_vp.join("/") + "/";
            debug!("sense_vp:{}",&sense_vp);
            //==========================================================
            if sense_vp.find(&_search_dir).is_some() {
                _return = _vp;
                debug!("sense_find%{}%",&_vword[1]);
                break 'outer;
            }
        }
    }
    return _return.to_string()
}
// === End Of Line ===
