/*
 * parse/syntax_test.rs
 *
 * ftml - Convert Wikidot code to HTML
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Tests for the parser.
//! This does not check any formatting, simply whether the grammar file
//! is correctly interpreting strings. So creating an invalid color or
//! inlined HTML is not an error here.
//!
//! This will be slowly converted and deprecated in favor of ast_test.

use super::{parse, Rule, WikidotParser};
use crate::handle::TestHandle;
use crate::prefilter;
use pest::Parser;

const VALID_INPUT_STRINGS: [&str; 133] = [
    "__**test** cherry {{ durian (?) }}__ ^^up!^^",
    " [ left bracket",
    "right bracket ] ",
    "** [[date 0]] **",
    "[[span class = \"test\"]]//hello// world![[footnote]]actually country[[/footnote]][[/span]]",
    "--[[*user rounderhouse]] [[# test-anchor ]]-- [[ eref equation_id ]]",
    "  [[ * user rounderhouse ]] [[ user aismallard ]] [[        user        rounderhouse        ]]  ",
    "[[ image tree.png link = \"https://example.com\" alt=\"A tree.\" class=\"image-block\"  ]]",
    "[[image file.jpeg]] [[image :first]] [[image https://example.com/picture.png]]",
    "[[<image left-aligned.png]] [[>image right-aligned.png]] [[=image centered.png]]",
    "[[ < image picture.png ]] [[ > image picture.png ]] [[ = image picture.png ]]",
    "[[f<image float-left.png ]] [[f>image float-right.png]]",
    "__**--^^,,{{super formatted}},,^^--**__",
    "//// Empty italics",
    "**** Empty bold",
    "____ Empty underline",
    "^^^^ Empty superscript",
    ",,,, Empty subscript",
    "//[[date -100]] number// [[footnote]]Content **cherry** [[*user aismallard]][[/footnote]] [[footnote]]Content **cherry** [[*user aismallard]][[/footnote]]",
    "[[>]]\n[[module Rate]]\n[[/>]]\n**Item #:** SCP-0000",
    "apple\n[[module Rate]]\nbanana",
    "apple\n[[module CSS]]\n@import url('https://example.com/style.css');\ndiv.container { display: none; }\n[[/module]]\nbanana",
    "[[form]]\nform data here\nmore stuff\n[[/form]]",
    "[[form]]\n[[/form]]",
    "[[note]]\nnote internal information here\napple\ndurian\nbanana\n[[/note]]",
    "apple\n[[note]]\ninternal\n[[/note]]\nbanana",
    "^^**alpha** beta ,,gamma,,^^",
    "apple\n----\nbanana\n-------\ncherry\n---------------\nkiwi",
    "apple\n~~~~\nbanana\n~~~~~~~\ncherry\n~~~~~~~~~~~~~~~\nkiwi",
    "apple\n~~~~>\nbanana\n~~~~<\ncherry\n~~~~\nkiwi",
    "= {{apple}} banana",
    "++ header\n+++ apple __banana__\n++++ @@ RAW @@\ndurian",
    "internal [[# anchor-name]] [[date 1000]] **apple** _",
    "apple [[span id=\"tag\" ]]banana[[/span]] __cherry__ [[span class=\"fruit-name\"]]pineapple [[span style=\"text-shadow: 2px 2px #f00;\"]]kiwi[[/span]] orange[[/span]] durian",
    "[[span id=\"a\"]] A [[ span id=\"b\"]] B [[span id=\"c\" ]] C [[ span id=\"d\" ]] D [[span  id =\"e\"]] E [[span  id  =  \"f\"]] F [[span id= \"g\"]] INNER [[/span]] [[/span]] [[/span]] [[/span]] [[/span]] [[/span]] [[/span]]",
    "[[span class=\"item\"]][[/span]]",
    "[[span]]apple\nbanana[[/span]]\n[[span]]\napple\n[[/span]]",
    "[[span class=\"apple\" ]] banana \n [[ span class=\"cherry\"]] kiwi \n [[ span class =\"durian\" ]] pineapple \n [[ span class = \"orange\" ]] test [[/span]] \n [[/span]] [[/span ]] [[/ span]] ",
    "fruit list: ##red|apple## ##dc143c|cherry## ## #0ff | ocean ## ###6495ed|blueberry##",
    "##black| alpha **beta** gamma^^2^^ __delta //epsilon//__ ## zeta",
    "//several {{layers //of {{formatting}}//}}//",
    "@@``@@ @@//@@ @@--@@ @@**@@ @@__@@ @@,,@@ @@^^@@ @@}}@@ @@{{@@ @@]]@@ @@[[@@ @@##@@ @@----@@ @@~~~~@@",
    "[[span id=\"email\"]] test.person@example.com [[/span]]",
    "[[date 1554823000]]\n[[ date 1554823000 ]]\n[[ date 1554823000 format=\"%A %B %d, %Y\" ]]\n[[date 1554823000  format = \"%A %B %d, %Y\"]]\n[[  date  1554823000  format= \"%A %B %d, %Y\"]]",
    "[[footnote]] Inner **contents** here [[date 0]] __please!__ [[/footnote]]",
    "[[footnote]] Multi-line\nfootnote\ncontents\nhere [[/footnote]]",
    "[[footnote]] \n APPLE \n BANANA \n CHERRY \n [[/footnote]]",
    "[[<]]\nleft-aligned **text**\n[[/<]]",
    "[[>]]\nright-aligned //text//\n[[/>]]",
    "[[=]]\ncenter-aligned __text__\n[[/=]]",
    "[[==]]\njustified {{text}}\n[[/==]]",
    "[[>]]\n[[module Rate]]\n[[/>]]\n[[=]]\n++ UNAUTHORIZED ACCESS IS __BAD__\ndon't do it\n[[/=]]",
    "[[==]]\n[[note]]\ninternal data here\n[[/note]]\nWas created on [[date 100000000]], thanks to [[*user rounderhouse]] for critique.\n##red|apple##\n[[/==]]",
    "[[>]]\nRIGHT\n[[<]]\nLEFT\n[[/<]]\nBLOCK\n[[/>]]",
    "[[code]]\nSome filenames:\n- Cargo.lock\n- Cargo.toml\n- LICENSE.md\n[[/code]]",
    "[[code type=\"CSS\"]]\n@charset 'utf-8';\n\n:root{\n    --theme-base: 'black-highlighter';    --theme-id: 'black-highlighter';\n}\n[[/code]]",
    "[[code lang= \"python\"]]\n[[/code]]\n$\n[[code language =\"RUST\"]]\nfn main() {\n    println!(\"Hello, world!\");\n}\n[[/code]]",
    "[[div class=\"raisa-notice\"]]\n++ RAISA NOTICE\nThis file is dank.\n[[/div]]",
    "[[ div id=\"apple\" class =\"banana\" style= \"display: none;\" ]]\ndurian\n[[/ div ]]",
    "[[div]]\n[[/div]]",
    "[[SPAN ID=\"apple\"]] [[FOOTNOTE]]cherry[[/FOOTNOTE]] [[/SPAN]] [[DATE 1000000 FORMAT=\"%A\"]]",
    "[[ IMAGE tree.png LINK = \"https://example.com\" ALT=\"A tree.\" CLASS=\"image-block\"  ]]",
    "[[IMAGE tree.png ]] [[<IMAGE left-aligned.png]] [[>IMAGE right-aligned.png]]",
    "[[f<IMAGE left-aligned.png]] [[f>IMAGE right-aligned.png]] [[=IMAGE centered.png]]",
    "[[ f<image left-aligned.png ]] [[ f>image right-aligned.png ]] [[ =image centered.png ]]",
    "[[NOTE]]\ncontents\n[[/NOTE]]\n[[CODE]]\ncontents\n[[/CODE]]\n[[DIV STYLE=\"display: none;\"]]\ncontents\n[[/DIV]]",
    "[[tabview]]\n[[tab Alpha]]\nIn the year 2012, a **great** calamity occurred...\n[[/tab]]\n[[tab Beta]]\n[[date 8000000]] lol what\n[[/tab]]\n[[/tabview]]",
    "[[tabview]][[/tabview]]",
    "[[tabs]]\n[[tab Alpha]][[/tab]]\n[[tab Beta]]\n[[/tab]]\n[[tab Gamma Delta]]\ndurian\n[[/tab]]\n[[/tabs]]",
    "[[tablist]] [[tab --alpha-- ]] beta [[/tab]] [[tab GAMMA]] [[/tab]] [[ TAB __delta ]][[/tab]] \n [[/tablist]]",
    "[[tabview]]\n[[tab A]]\n[[tablist]]\n[[tab B]][[/tab]]\n[[/tablist]]\n[[/tab]]\n[[/tabview]]",
    "[[TABVIEW]][[/TABVIEW]][[TABS]][[/TABS]][[TABLIST]][[/TABLIST]]",
    "apple [[gallery]] banana",
    "[[GALLERY]] - The AWCY exhibit that yells at you",
    "apple\n+ h1\n++ h2\n+++ h3\n++++ h4\n+++++ h5\n++++++ h6\nbanana",
    "+++ stylized **heading** right //here//!",
    "[[div class=\"test\"]]\n+++ NOTICE FROM RAISA OR SOMETHING\n[[/div]]",
    "[[size 50%]] alpha [[/size]] - [[size x-large]] beta [[/size]] - [[ size 2rem ]]gamma[[/ size ]]",
    "[[size 120%]]\napple\nbanana\n[[/size]] [[SIZE 1EM]]cherry[[/SIZE]]",
    "* apple\n* __banana__\n*  cherry\n*   durian\n",
    "# alpha\n# __beta__\n# gamma\n#  delta\n#   epsilon\n",
    "* apple\n# banana\n* cherry\n# durian",
    "* one\n * two\n  * three\n  * three\n   * four\n",
    "[[quote]]\nQuoted text here\n[[/quote]]",
    "[[quote id=\"my-id\" style=\"line-height: 1.5em;\" class=\"raisa-notice\"]]\n[[/quote]]",
    "[[QUOTE]]\nNested quotes are easier this way\n[[QUOTE]]\nvery deep\nindeed\n[[/QUOTE]]\n[[/QUOTE]]",
    "[[ quote class = \"quote-block level-1\" ]]\ncontents\n[[ quote class = \"quote-block level-2\" ]]\napple\n[[/ quote ]]\nbanana\n[[/ quote ]]",
    "[[js]]\nfunction test() { return 1; }\n[[/js]]\n[[ JS ]]\n[[/ JS ]]",
    "**apple**\n[[javascript]]\nconsole.log('test');\n[[/javascript]]\n**banana**",
    "Email me at person@example.com! Check out my [[[example-author-page | amazing author page]]]!",
    "[[[page-with-no-name]]] [[[*new-tab-with-no-name]]] [[[ * How To Write an SCP ]]] ",
    "[[[https://example.com | Example]]] [[[*https://example.com | This one opens in a new tab!]]] yay",
    "Bare link: https://example.com/ Named link: [https://example.com/ example site]",
    "New tab bare link: *https://example.com/page1.html New tab link: [*https://example.com/page2.html bidoof]",
    "Named link with spaces: [  http://some-http-site.com/use-https-folks a link  ] ",
    "Weird link with spaces: [  /category:thing/page/idk name  ] ",
    "Anchor with spaces: [[ a href = \"https://google.com\" id = \"test\" ]] contents [[/ a ]]",
    "ANCHOR WITH SPACES: [[ A HREF = \"https://google.com\" ID = \"TEST\" ]] CONTENTS [[/ A ]]",
    "[# empty link] [/category:thing/page/idk bottom text] [*/category:thing/page gamers against weed]",
    "[[a href=\"https://example.com/\" id=\"test\" style=\"color: blue\"]] **anchor link!** [[/a]]",
    "[[a_ href=\"https://example.com/\" name=\"dumb-test\"]] not sure why these exist but whatever [[/a_]]",
    "[[# anchor-name-1]] [[ a name = \"anchor-name-2\" ]] [[/a]] [[a name=\"anchor-name-3\"]][[/a]]",
    "[[[ link \"TO\" a; <pagE> ]]] [[[ some page | ]]] [[[/ | root]]] [[[page#toc1]]]",
    "GoI-something [https://en.wikipedia.org/wiki/Military%E2%80%93industrial_complex PENTAGRAM]",
    "[[=]]\n[[collapsible]]\n[[/collapsible]]\n[[/=]]\n",
    "[[collapsible show=\"▸ Access File\" hide=\"▾ Close File\"]]\ninterview info data here\nalpha\n[[/collapsible]]",
    "[[ COLLapsiBLE ]]\n* **Threat Level:** ##green|Green##\n* **Threat Level:** ##yellow|Banana##\n[[/ collAPSIBLe ]]",
    "{\n[[collapsible hideLocation = \"both\" HIDE = \"> hide <\"]]\napple banana cherry\npineapple durian\n[[/collapsible]]\n}",
    "[[collapsible id=\"coll-id\" style=\"text-align: center;\" class=\"classified-box\" HIDELOCATION=\"bottom\"]]\nalpha __banana__\n**cherry**\n[[/collapsible]]",
    "[[html]]\nsome <b>raw</b> html here! <br> all my own <span class=\"test\">stuff</span>\n[[/html]]",
    "\n[[iframe https://example.com/]]\n",
    "[[iframe https://example.com/some/page name= \"test\" src =\"https://example.com\" ]]\n",
    "[[ IFRAME https://scp-wiki.net/scp-001 data-qa = \"test--iframe\" sandbox=\"allow-scripts\" ]]",
    "[[file some_image.png]]",
    "[[* FILE /dir/file ]]",
    "[[ file  some_image.png | link text ]]",
    "[[css]]\nsome-class {\n  display: none;\n}\n[[/css]]",
    "[[ STYLE ]]\na:hover {\n  color: blue;\n}\n[[/ STYLE ]]",
    "[[b]]apple[[/b]] banana [[ B ]]_[[/b  ]]",
    "[[bold]]cherry durian[[/bold]] [[ BOLD ]]_[[/ BolD  ]]",
    "[[i]]pineapple[[/i]] banana [[ I ]]_[[/ I ]]",
    "[[italics]] --durian-- [[/italics]] [[ ITALICS ]]_[[/ ItaLiCs  ]]",
    "[[u]]AAA[[/u]] ~ [[underline]]BBB[[/underline]]",
    "[[sub]] ##green|sub## [[/sub]] [[ SUB ]]_[[/ SUB  ]]",
    "[[subscript]] ##blue|sub## [[/subscript]] [[ SUBSCRIPT ]]_[[/ SUBScripT  ]]",
    "[[sup]] ##green|super## [[/sup]] [[ SUP ]]_[[/ SUP  ]]",
    "[[superscript]] ##blue|super## [[/superscript]] [[ SUPERSCRIPT ]]_[[/ SUPErScripT  ]]",
    "[[mono]] M O N O S P A C E [[/mono]] [[ MONO ]]_[[/ MONO ]]",
    "[[monospace]] M O N O S P A C E [[/monospace]] [[ MOnoSPaCE ]]_[[/ MONOspaCe  ]]",
    "**Item #:** [[title]]",
    "[[alt_title]] - by aismallard",
    "++ Property of [[header]]",
    "--- [[subheader]] ---",
];

const VALID_FILTER_STRINGS: [&str; 12] = [
    "",
    "[!-- [[ footnote invalid formatting in here-- [[ eref --] test",
    "__ [[  date 0  ]] [!-- comment here --]__",
    "something with nothing to filter",
    "> hello world\n> my name is john\n> I like long walks on the beach\n> and writing scips\n",
    ">this implementation doesn't require spaces after the '>' because we're not lame",
    "> [[span style = \"color: blue;\"]] blue text! [[/span]] lol\n> test",
    "> [[div class=\"test\"]]\n> cherry\n> pineapple\n> [[/div]]",
    "the following document was found:\n> oh no many bad thing\n>> execute the order\n> it no good\n",
    ">>>>> very deep quote block\n>>>>> again\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> even deeper",
    "----\n---\n--",
    " omg... he actually did it ",
];

#[test]
fn test_valid_strings() {
    // Parse only
    for string in &VALID_INPUT_STRINGS[..] {
        println!("Parsing valid string: {:?}", string);
        if let Err(err) = WikidotParser::parse(Rule::page, string) {
            panic!(
                "Failed to parse test string:\n{}\n-----\nProduced error: {}",
                string, err
            );
        }
    }

    // Parse and make SyntaxTree
    for string in &VALID_INPUT_STRINGS[..] {
        println!("Converting valid string: {:?}", string);
        if let Err(err) = parse(string) {
            panic!(
                "Failed to convert test string:\n{}\n-----\nProduced error: {}",
                string, err
            );
        }
    }
}

#[test]
fn test_valid_filter_strings() {
    let mut buffer = String::new();

    for string in &VALID_FILTER_STRINGS[..] {
        println!("Running prefilter test on valid string: {:?}", string);
        buffer.push_str(string);
        prefilter(&mut buffer, &TestHandle).expect("Prefilter shouldn't be failing");

        if let Err(err) = WikidotParser::parse(Rule::page, &buffer) {
            panic!(
                "Failed to parse filtered test string:\n{}\n-----\nProduced error: {}",
                string, err
            );
        }

        if let Err(err) = parse(&buffer) {
            panic!(
                "Failed to convert filtered test string:\n{}\n-----\nProduced error: {}",
                string, err
            );
        }

        buffer.clear();
    }
}
