
#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter, StaticFilesHandler, FaviconHandler};
use std::collections::HashMap;

// Needed For Post
extern crate hyper;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
// Needed For Post

fn main() {

    //Implement A Server
    let mut server = Nickel::new();

    //Route Using Mustache Templateing Agent
    server.get("/", middleware! { |req, mut res|

        let mut data = HashMap::<&str, &str>::new();
        data.insert("tab-title","Verge | Web Development, Design, and SEO Company In Tucson");
        data.insert("title","<span style=\"margin-left: 28.9px; position: absolute; margin-top: 17px;vertical-align: middle;\">V</span><span style=\"font-size: 145%; color: #C36159;vertical-align: middle;\">&#9678</span><span style=\"margin-left: -7px; vertical-align: middle;\">erge</span></h1>");
        data.insert("sub-title", "Web Development");
        data.insert("hint", "<div id=\"hint\"><span id=\"message\">Use Arrow Keys</span><div>&#x276f;</div></div>");

        return res.render("views/index.hbs", &data);
    });


    server.get("/contact", middleware! { |req, mut res|

        let mut data = HashMap::new();
        data.insert("tab-title","Contact Us | Web Development, Design, and SEO Company In Tucson");
        data.insert("title","Contact");

        // let mut s = String::new();
        // req.origin.read_to_string(&mut s);

        return res.render("views/contact.hbs", &data)
    });


    server.post("/confirmation", middleware!{ |req, res|

        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data);

        println!("{}", form_data);

        // let mut data = HashMap::new();
        // data.insert("tab-title","Submit | Verge Web Development, Design, and SEO Company In Tucson");
        // data.insert("title","Submit");
        // data.insert("confirmation", &the);
        // return res.render("views/confirmation.hbs", &data)
    });


    server.get("/about", middleware!{ |req, res|

        let mut data = HashMap::new();
        data.insert("tab-title","About");
        data.insert("title","About");

        return res.render("views/about.hbs", &data);
    });


    //Serve A Static Directory
    server.utilize(StaticFilesHandler::new("public/"));
    //server.utilize(FaviconHandler::new("/images/file"));



    server.listen("0.0.0.0:3000");
}
