use std::io;

#[cfg(feature = "naive")]
fn countlines()->usize{

    let mut cnt = 0;
    let lines = io::stdin().lines();
    for _line in lines {
        cnt+= 1; 
    }
    cnt
}

#[cfg(feature = "explicit_lock")]
fn countlines()->usize{
    use std::io::BufRead;
    let mut cnt = 0;
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    loop {
        let r = stdin.read_line(&mut buffer);
        match r {
            Ok(x) => {
                if x==0{
                    break;
                }
                cnt += 1;
            }
            Err(_) => {break;}
        }
        buffer.clear();
    }
    cnt
}

#[cfg(feature = "read_until")]
fn countlines()->usize{
    use std::io::BufRead;
    let mut stdin = io::stdin().lock();
    let mut cnt = 0;
    let mut buf:Vec<u8> = Vec::with_capacity(2048);
    loop {
        let num_bytes = stdin.read_until(b'\n', &mut buf).expect("reading from stdin won't fail");
        if num_bytes ==0 {
            break;
        }
        cnt += 1;
        buf.clear();
    }
    cnt
}

#[cfg(feature = "linereader")]
fn countlines()->usize{
    use linereader::LineReader;
    let mut cnt = 0;
    let mut reader = LineReader::with_capacity(1024*1024, io::stdin().lock());
    reader.for_each(|_line| {
        cnt +=1;
        Ok(true)
    }).expect("YOLO");
    cnt
}

#[cfg(feature = "wordcount")]
fn countlines()->usize{
    use std::io::BufRead;
//    use memchr;
    let mut cnt:usize = 0;

    let mut stdin = io::stdin().lock();

    loop{
        let used = {
            let available = match stdin.fill_buf() {
            Ok(n) => n,            
            Err(_e) => {panic!("AAA")},
            };
            if available.len() == 0 {
                break;
            }
            /*if cfg!(charsearch=memchr)
            {
                for _ in memchr::memchr_iter('\n' as u8, available) {
                    cnt += 1;
                }
            }
            else*/
            {
                for b in available{
                    cnt += (*b == ('\n' as u8)) as usize;
                }
            }//*/
            available.len()
        };
        stdin.consume(used);
    }
    cnt
}

fn main() {
    let cnt = countlines();
    println!("Done reading {} lines", cnt);
}
