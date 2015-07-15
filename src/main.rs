// tool to inject guassian noise into an image

extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::env;
extern crate rand;
use rand::distributions::{Normal, IndependentSample};


pub struct ImageOptions {
    tiffFilename  : String,
    imageWidth    : u16,
    imageHeight   : u16,
    startingCode  : u16,
    endingCode    : u16,
    sequenceCount : u16,
    increment     : bool,
    lineNoise     : f64,
    columnFPN     : f64,
    columnNoise   : f64,
}

impl ImageOptions {
     fn new() -> ImageOptions {
         ImageOptions {
             tiffFilename  : "".to_string(),
             imageWidth    : 0u16,
             imageHeight   : 0u16,
             startingCode  : 0u16,
             endingCode    : 0u16,
             sequenceCount : 0u16,
             increment     : false,
             lineNoise     : 0.0f64,
             columnFPN     : 0.0f64,
             columnNoise   : 0.0f64,
         }
     }

    fn mnew( matches : &getopts::Matches ) -> ImageOptions {
        let args: Vec<String> = env::args().collect();
        let w_h_good = (
                ( 2  == matches.free.len()       )
            &&  ( args[ 1 ] == matches.free[ 0 ] )
            &&  ( args[ 2 ] == matches.free[ 1 ] )
        );
        ImageOptions {
            tiffFilename  : matches.opt_str( "f" ).unwrap_or( "".to_string() ), // "".to_string(),
            imageWidth    : matches.free[ 0 ].trim().parse::<u16>().ok().unwrap_or(   0u16 ), // 0u16,
            imageHeight   : matches.free[ 1 ].trim().parse::<u16>().ok().unwrap_or(   0u16 ), // 0u16,
            startingCode  : matches.opt_str( "s" ).unwrap_or( "0".to_string() ).trim().parse::<u16>().ok().unwrap_or(   0u16 ), // 0u16,
            endingCode    : matches.opt_str( "e" ).unwrap_or( "0".to_string() ).trim().parse::<u16>().ok().unwrap_or(   0u16 ), // 0u16,
            sequenceCount : matches.opt_str( "q" ).unwrap_or( "0".to_string() ).trim().parse::<u16>().ok().unwrap_or(   0u16 ), // 0u16,
            increment     : matches.opt_present( "i" ),
            lineNoise     : matches.opt_str( "l" ).unwrap_or( "0".to_string() ).trim().parse::<f64>().ok().unwrap_or( 0.0f64 ), // 0.0f64,
            columnFPN     : matches.opt_str( "c" ).unwrap_or( "0".to_string() ).trim().parse::<f64>().ok().unwrap_or( 0.0f64 ), // 0.0f64,
            columnNoise   : matches.opt_str( "n" ).unwrap_or( "0".to_string() ).trim().parse::<f64>().ok().unwrap_or( 0.0f64 ), // 0.0f64,
        }
    }

    fn print( &self) {
        println!("The following image options will be used: " );
        println!("tiffFilename  : {:?}", self.tiffFilename    );
        println!("imageWidth    : {:?}", self.imageWidth      );
        println!("imageHeight   : {:?}", self.imageHeight     );
        println!("startingCode  : {:?}", self.startingCode    );
        println!("endingCode    : {:?}", self.endingCode      );
        println!("sequenceCount : {:?}", self.sequenceCount   );
        println!("increment     : {:?}", self.increment       );
        println!("lineNoise     : {:?}", self.lineNoise       );
        println!("columnFPN     : {:?}", self.columnFPN       );
        println!("columnNoise   : {:?}", self.columnNoise     );
        println!("========================================= " );
    }

}


fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} <columns> <rows> <-f|-s|-e>", program);
    print!("{}", opts.usage(&brief));
}

fn cmdline_options( args : &Vec<String> ) -> Option< Matches > {
    let mut opts = Options::new();
    opts.optopt("f", "file", "Tiff file name to use", "SOMENAME.TIF");
    opts.optopt("s", "starting_code", "Starting Code for the image", "STARTIG_CODE");
    opts.optopt("e", "ending_code",     "Ending Code for the image. ",                    "END_CODE_IN_LSB"   );
    opts.optopt("l", "line_noise",      "Line noise in the image. ",                      "LINE_NOISE_IN_LSB" );
    opts.optopt("c", "column_fpn",      "Column FPN in the image. ",                      "CFPN_IN_LSB"       );
    opts.optopt("n", "column_tn",       "Column temporal noise in the image. ",           "DEFAULT"           );
    opts.optopt("q", "create_sequence", "Create a sequence of images. Provide the count after this option", "DEFAULT" );
    opts.optflag("i", "increment_code", "Increment with each count. Use with -q option."                      );
    opts.optflag("h", "help",           "print this help menu"                                                );

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); print_usage(&program, &opts); }
    };

    if (
             matches.opt_present("h")
        ||   ( 4 > args.len() )
    )
    {
        print_usage(&program, &opts);
        return None;
    }
    return ( Some ( matches ) );
}

fn CreateADCImage( imageOptions: &ImageOptions, colFPNSamples: Vec<f64> ) -> bool {


    return false;
}

fn GaussianRandomNum( normal : & Normal ) -> f64 {
    let v = normal.ind_sample(&mut rand::thread_rng());
    return v;
}

fn CreateSingleImage( imageOptions: &ImageOptions ) -> bool {
    // let mut j = 0u16;
    // double* colFPNSamples = (double*)malloc(sizeof(double) * (imageOptions.imageWidth));
    // let mut colFPNSamples : Vec<f64> = Vec::with_capacity( imageOptions.imageWidth );

    // for ( j = 0; j < imageOptions.imageWidth; j++ )
    // {
    //     colFPNSamples[j] = GaussianRandomNum(imageOptions.columnFPN);
    // }
    let normal = Normal::new(0.0, imageOptions.columnFPN );
    let colFPNSamples = ( 0..imageOptions.imageWidth).map( | _j | GaussianRandomNum( &normal ) ).collect::<Vec<_>>();
    println!("colFPNSamples = {:?}", colFPNSamples );
    return CreateADCImage(imageOptions, colFPNSamples);
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = cmdline_options( &args ).expect( "" );
    let imageOptions = ImageOptions::mnew( &matches );
    imageOptions.print();
    // if ( imageOptions.sequenceCount <= 1 )
        CreateSingleImage( &imageOptions );
    // else
    //     CreateImageSequence( &imageOptions );
}