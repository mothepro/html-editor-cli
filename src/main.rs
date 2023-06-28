use clap::Parser;
use html_editor::operation::*;
use html_editor::{parse, Node};

use std::io::{self, BufRead};

/// Args to replace some html
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Document query for the element to replace.
    #[arg(short, long)]
    query: String,

    /// The attribute to be given the new content.
    /// If left unset, the content will replace the element's children.
    #[arg(short, long)]
    attribute: Option<String>,

    /// The content to use in the replacement.
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

    let mut dom = parse(&input).unwrap();
    let query = args.query.as_str();
    let selector = Selector::from(query);

    dom.execute_for(&selector, |element| {
        let value = args.content.to_owned();

        if args.attribute.is_some() {
            let attr = args.attribute.as_ref().unwrap().to_owned();
            let mut new_attr = vec![(attr, value)];
            element.attrs.append(&mut new_attr);
        } else {
            let mut new_child = vec![Node::Text(value)];
            element.children.clear();
            element.children.append(&mut new_child);
        }
    });

    println!("{}", dom.html());
}
