use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, parse_input};
use std::collections::HashMap;

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

struct Replacer {
    list: Vec<Replacement>,
}

impl Replacer {
    fn new(ctx: &PreprocessorContext) -> Self {
        Self {
            list: get_replace_table(&ctx.config),
        }
    }
}

impl Preprocessor for Replacer {
    fn name(&self) -> &'static str {
        "replace"
    }

    fn run(
        &self,
        _ctx: &PreprocessorContext,
        mut book: Book,
    ) -> mdbook_preprocessor::errors::Result<Book> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chap) = item {
                for replacement in &self.list {
                    chap.content = chap.content.replace(&replacement.from, &replacement.to);
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> mdbook_preprocessor::errors::Result<bool> {
        Ok(true)
    }
}

fn get_replace_table(config: &Config) -> Vec<Replacement> {
    let Ok(Some(table)) = config.get::<HashMap<String, String>>("preprocessor.replace.list") else {
        return Vec::new();
    };

    table
        .into_iter()
        .map(|(key, value)| Replacement {
            from: key,
            to: value,
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("supports") {
        return Ok(());
    }

    let (ctx, book) = parse_input(std::io::stdin())?;
    let book = Replacer::new(&ctx).run(&ctx, book)?;
    serde_json::to_writer(std::io::stdout(), &book)?;
    Ok(())
}
