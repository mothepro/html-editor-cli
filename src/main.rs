use clap::Parser;
use html_editor::{operation::*};
use html_editor::{parse, Node};

use std::io::{self, BufRead};

/// Args to replace some html
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Document query for the element to replace
   #[arg(short, long)]
   query: String,

   /// The attribute to replace with this value
   /// if it is not set, use the innerHTML
   #[arg(short, long)]
   attribute: String,

   /// The content to be replaced with
   #[arg(short, long)]
   content: String,
}

fn main() {
   let args = Args::parse();

    let mut input = String::new();
    for line in io::stdin().lock().lines() {
        if let Ok(str) = line {
            input.push_str(&str);
        }
    }

    let dom = parse(&input).unwrap();
    let selector = Selector::from(&args.query);

    println!("size = {}", dom.query_all(&selector).len());
    for mut element in dom.query_all(&selector) {
            println!("no a = {}\n", element.html());

        let value = args.content.to_owned();
        let attr = args.attribute.to_owned();
        if attr.is_empty() {
            let inner_html_replacement = Node::new_element(
                "span",
                vec![],
                vec![Node::Text(value)]
            );
            element.insert_to(&selector, inner_html_replacement);
            println!("no a = {}", element.html());

        }else {
            println!("a({}) = {}",attr, element.html());
            element.attrs.push((attr, value));
            println!("a({}) = {}",element.attrs.len(), element.html());

        }
    }

    println!("{}", dom.html());
}
