use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
struct CmdArgs {
    #[arg(short, long)]
    items: u8,

    #[arg(short, long)]
    radius: Option<f32>,

    #[arg(short, long)]
    diameter: Option<f32>,

    #[arg(short, long)]
    xstart: f32,

    #[arg(short, long)]
    ystart: f32,
}

fn main() {
    pretty_env_logger::init();
    let mut args = CmdArgs::parse();
    if args.radius == None && args.diameter == None {
        eprintln!("must provide either radius or diameter");
        return;
    } else if args.radius == None {
        args.radius = Some(args.diameter.unwrap()/2.0);
    } else if args.diameter == None {
        args.diameter = Some(args.radius.unwrap()*2.0);
    }
    println!("cmd args {:#?}", args);

    let staring_angle:f32 = 0.0;

    for i in 0..args.items {
        let angle = (2.0 * 3.14 * (i as f32)/args.items as f32) + staring_angle;
        println!("Degree: {}", angle*180.0/3.14);
        let px = args.xstart + args.radius.unwrap() * angle.cos();
        let py = args.ystart + args.radius.unwrap() * angle.sin();
        println!("# {}, px: {}  py: {}", i, px, py);
    }
}
