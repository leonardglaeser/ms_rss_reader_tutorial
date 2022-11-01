use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};
use windows::Web::Syndication::{
ISyndicationText, RetrievalProgress, SyndicationClient, SyndicationFeed, SyndicationItem,
};
use windows::Foundation::Uri;

fn main() -> Result<()> {
    let uri = Uri::CreateUri(h!("https://blogs.windows.com/feed"))?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(&uri)?.get()?;

    let mut titles: Vec<String> = Vec::new();
    for item in feed.Items()?
    {

        let heading = item.Title()?.Text()?;
        println!("{}", heading);
        titles.push(heading.to_string());
    }

    let feedtext = titles.join("\n");
    println!("\n\n\n{}",feedtext); //professional debuggibg
    let lptext: PCSTR = PCSTR::from_raw(feedtext.as_bytes().as_ptr());

    // There is something wrong!!!!!
    unsafe {
        //lptext contains additional chars ?!
        MessageBoxA(None,lptext, s!("RSS_News"), MB_OK);
    }


    Ok(())
}