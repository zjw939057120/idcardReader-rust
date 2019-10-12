extern crate libloading;
extern crate base64;
extern crate encoding;
extern crate file;
extern crate image;

use std::os::raw::{c_char, c_int, c_uchar};
use std::io;
use libloading::{Library, Symbol};
use base64::{decode, encode};
use encoding::{DecoderTrap, Encoding};
use encoding::all::UTF_16LE;

fn main() {
    let port: c_int = 1001;//端口号
    let if_open: c_int = 0;//是否内部打来
    let mut mana_info: [c_uchar; 4] = [b'0'; 4];//找卡结果
    let mut mana_msg: [c_uchar; 8] = [b'0'; 8];//选卡结果
    let mut ch_msg: [c_uchar; 256] = [b'0'; 256];//身份证文字信息
    let mut ch_msg_len = 0;//文字信息长度
    let mut ph_msg: [c_uchar; 1024] = [b'0'; 1024];//身份证相片信息
    let mut ph_msg_len: c_int = 0;//相片信息长度
//    let mut img_msg: [c_uchar; 38556] = [b'0'; 38556];//相片原始数据
 let res = reader();
    println!("{:?}", res);

}

fn reader<'a>() -> &'a str {
    let port: c_int = 1001;//端口号
    let if_open: c_int = 1;//是否内部打来
    let mut mana_info: [c_uchar; 4] = [b'0'; 4];//找卡结果
    let mut mana_msg: [c_uchar; 8] = [b'0'; 8];//选卡结果
    let mut ch_msg: [c_uchar; 256] = [b'0'; 256];//身份证文字信息
    let mut ch_msg_len = 0;//文字信息长度
    let mut ph_msg: [c_uchar; 1024] = [b'0'; 1024];//身份证相片信息
    let mut ph_msg_len: c_int = 0;//相片信息长度
    let mut img_msg: [c_uchar; 38556] = [b'0'; 38556];//相片原始数据


    let lib = Library::new("sdtapi").unwrap();

    //关闭端口
    unsafe {
        let close_port: Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"SDT_ClosePort").unwrap();
        let res = close_port(port);
//        println!("关闭{:x}", res);
    }
    //打开端口
    unsafe {
        let open_port: Symbol<unsafe extern fn(c_int) -> c_int> = lib.get(b"SDT_OpenPort").unwrap();
        let res = open_port(port);
//        println!("打开状态：{:?}", res);
        if res != 144
        {
            return "{\"ret\":0,\"msg\":\"打开端口失败\",\"data\":\"\"}";
        }
    }

    //找卡
    unsafe {
        let find_idcard: Symbol<unsafe extern fn(c_int, *mut c_uchar, c_int) -> c_int> = lib.get(b"SDT_StartFindIDCard").unwrap();
        let res = find_idcard(port, mana_info.as_mut_ptr(), if_open);
//        println!("找卡状态：{:?}", res);
        if res != 159
        {
            return "{\"ret\":0,\"msg\":\"查找身份证失败\",\"data\":\"\"}";
        }
    }

    //选卡
    unsafe {
        let select_idcard: Symbol<unsafe extern fn(c_int, *mut c_uchar, i32) -> c_int> = lib.get(b"SDT_SelectIDCard").unwrap();
        let res = select_idcard(port, mana_msg.as_mut_ptr(), if_open);
//        println!("选卡状态：{:?}", res);
        if res != 144
        {
            return "{\"ret\":0,\"msg\":\"选择身份证失败\",\"data\":\"\"}";
        } else {
            //   return "{\"ret\":1,\"msg\":\"选择身份证失败\",\"data\":\"\"}";
        }
    }

    //读卡
    unsafe {
        let read_idcard: Symbol<unsafe extern fn(c_int, *mut c_uchar, *mut c_int, *mut c_uchar, *mut c_int, c_int) -> c_int> = lib.get(b"SDT_ReadBaseMsg").unwrap();
        let res = read_idcard(port, ch_msg.as_mut_ptr(), &mut ch_msg_len as *mut c_int, ph_msg.as_mut_ptr(), &mut ph_msg_len as *mut c_int, if_open);
//        println!("读卡状态：{:?}", res);
        if res != 144
        {
            return "{\"ret\":0,\"msg\":\"读取身份证失败\",\"data\":\"\"}";
        }
    }

    //关闭端口
    unsafe {
        let close_port: Symbol<unsafe extern fn(i32) -> i32> = lib.get(b"SDT_ClosePort").unwrap();
        let res = close_port(port);
//        println!("关闭{:x}", res);
    }


    //文字解码
    let Name = &ch_msg[0..30];//姓名,长度30
    let Sex = &ch_msg[30..32];//性别,长度2
    let Nation = &ch_msg[32..36];//民族,长度4
    let Birth = &ch_msg[36..52];//出生,长度16
    let Addr = &ch_msg[52..122];//住址,长度70
    let Id = &ch_msg[122..158];//公民身份号码,长度36
    let Dept = &ch_msg[158..188];//签发机关,长度30
    let Start = &ch_msg[188..204];//有效期起始日期,长度16
    let End = &ch_msg[204..220];//有效期截止日期,长度16

    let name = UTF_16LE.decode(Name, DecoderTrap::Replace).unwrap();
    let sex = show_sex(UTF_16LE.decode(Sex, DecoderTrap::Replace).unwrap());
    let nation = show_nation(UTF_16LE.decode(Nation, DecoderTrap::Replace).unwrap());
    let birth = UTF_16LE.decode(Birth, DecoderTrap::Replace).unwrap();
    let addr = UTF_16LE.decode(Addr, DecoderTrap::Replace).unwrap();
    let id = UTF_16LE.decode(Id, DecoderTrap::Replace).unwrap();
    let dept = UTF_16LE.decode(Dept, DecoderTrap::Replace).unwrap();
    let start = UTF_16LE.decode(Start, DecoderTrap::Replace).unwrap();
    let end = UTF_16LE.decode(End, DecoderTrap::Replace).unwrap();
//    println!("name：{}", name);
//    println!("sex：{}", sex);
//    println!("nation：{}", nation);
//    println!("birth：{}", birth);
//    println!("addr：{}", addr);
//    println!("id：{}", id);
//    println!("dept：{}", dept);
//    println!("start：{}", start);
//    println!("end：{}", end);

    let libwlt = Library::new("DLL_File").unwrap();
    //相片解码
    unsafe {
        let unpack: Symbol<unsafe extern fn(*mut c_uchar, *mut c_uchar, c_int) -> c_int> = libwlt.get(b"unpack").unwrap();
        let res = unpack(ph_msg.as_mut_ptr(), img_msg.as_mut_ptr(), 1);
//        println!("解码状态：{}", res);
    }

    let pic = file::get("zp.bmp").unwrap();
    return string_to_static_str("{\"ret\":1,\"msg\":\"读取身份证成功\",\"data\":{\"name\":\"".to_string() + name.as_str() +
        "\",\"sex\":\"" + sex +
        "\",\"nation\":\"" + nation +
        "\",\"birth\":\"" + birth.as_str() +
        "\",\"addr\":\"" + addr.as_str() +
        "\",\"id\":\"" + id.as_str() +
        "\",\"dept\":\"" + dept.as_str() +
        "\",\"start\":\"" + start.as_str() +
        "\",\"end\":\"" + end.as_str() +
        "\",\"image\":\"" + "data:image/bmp;base64," + encode(&pic).as_str() +
        "\"}}");
}



fn string_to_static_str<'a>(s: String) -> &'a str {
    Box::leak(s.into_boxed_str())
}

fn show_sex<'a>(sex: String) -> &'a str {
    let sex = match &sex[..] {
        "1" => "男",
        "2" => "女",
        _ => "其他",
    };
    sex
}

fn show_nation<'a>(nation: String) -> &'a str {
    let nation = match &nation[..] {
        "01" => "汉",
        "02" => "蒙古",
        "03" => "回",
        "04" => "藏",
        "05" => "维吾尔",
        "06" => "苗",
        "07" => "彝",
        "08" => "壮",
        "09" => "布依",
        "10" => "朝鲜",
        "11" => "满",
        "12" => "侗",
        "13" => "瑶",
        "14" => "白",
        "15" => "土家",
        "16" => "哈尼",
        "17" => "哈萨克",
        "18" => "傣",
        "19" => "黎",
        "20" => "傈僳",
        "21" => "佤",
        "22" => "畲",
        "23" => "高山",
        "24" => "拉祜",
        "25" => "水",
        "26" => "东乡",
        "27" => "纳西",
        "28" => "景颇",
        "29" => "柯尔克孜",
        "30" => "土",
        "31" => "达斡尔",
        "32" => "仫佬",
        "33" => "羌",
        "34" => "布朗",
        "35" => "撒拉",
        "36" => "毛南",
        "37" => "仡佬",
        "38" => "锡伯",
        "39" => "阿昌",
        "40" => "普米",
        "41" => "塔吉克",
        "42" => "怒",
        "43" => "乌孜别克",
        "44" => "俄罗斯",
        "45" => "鄂温克",
        "46" => "德昂",
        "47" => "保安",
        "48" => "裕固",
        "49" => "京",
        "50" => "塔塔尔",
        "51" => "独龙",
        "52" => "鄂伦春",
        "53" => "赫哲",
        "54" => "门巴",
        "55" => "珞巴",
        "56" => "基诺",
        _ => "未知",
    };
    nation
}

