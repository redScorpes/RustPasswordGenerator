# RustPasswordGenerator

This is a simple CLI Password Generator App made in Rust which uses argument parsing via clap.

#### Options:
-l, --length <LENGTH>                  Length of the password  
-a, --all                              Include all possible characters  
-c, --lowercase letters                Lowercase letters include: abcdefghijklmnopqrstuvwxyz  
-u, --uppercase letters                Uppercase letters include: ABCDEFGHIJKLMNOPQRSTUVWXYZ  
-n, --numbers                          Numbers include: 0123456789  
-s, --special characters               Special characters include: !"#$%&'()*+,-./  
-z, --separator <SEPARATOR>            Separator between segments [default: -]  
-p, --segment-length <SEGMENT_LENGTH>  Length of each segment. 0 disabled the segment feature [default: 0]  
-h, --help                             Print help  
-V, --version                          Print version