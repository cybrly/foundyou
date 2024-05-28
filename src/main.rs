use clap::{Parser, ValueEnum};
use prettytable::{Table, Row, Cell, format};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "Find voter records by state and name.")]
struct Args {
    /// State code (e.g., -de for Delaware)
    #[clap(short, long, value_enum)]
    state: State,

    /// First name of the person
    first_name: String,

    /// Last name of the person
    last_name: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum State {
    AR, CO, DE, DC, FL, ID, MI, MO, MS, NJ, NY, NC, NV, OH, OK, PA, UT, VT, WA, WY
}

fn main() {
    let args = Args::parse();

    let state_urls: HashMap<&str, &str> = [
        ("AR", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("CO", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("DE", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("DC", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("FL", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("ID", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("MI", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("MO", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("MS", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("NJ", "https://stevemorse.hopto.org/njvoters/njvoters.php"),
        ("NY", "https://stevemorse.hopto.org/nysvoters/nysvoters.php"),
        ("NC", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("NV", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("OH", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("OK", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("PA", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("UT", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("VT", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("WA", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
        ("WY", "https://stevemorse.hopto.org/voterrecords/voterrecords.php"),
    ].iter().cloned().collect();

    let state_code = format!("{:?}", args.state);
    let url = state_urls.get(state_code.as_str()).unwrap();

    let url = match state_code.as_str() {
        "NJ" => format!(
            "{}?FirstNameKind=contains&FirstNameMax={}&MiddleNameKind=exact&MiddleNameMax=&LastNameKind=starts&LastNameMax={}&HouseNumberKind=exact&HouseNumberMax=&FractionKind=exact&FractionMax=&ApartmentKind=exact&ApartmentMax=&PreStreetDirectionKind=exact&PreStreetDirectionMax=&streetKind=exact&streetMax=&PostStreetDirectionKind=exact&PostStreetDirectionMax=&CityKind=contains&CityMax=&CountyKind=exact&CountyMax=&Zip5Kind=exact&Zip5Max=&MailAddress1Kind=exact&MailAddress1Max=&MailAddress2Kind=exact&MailAddress2Max=&MailAddress3Kind=exact&MailAddress3Max=&MailAddress4Kind=exact&MailAddress4Max=&DateofBirthKind=starts&DateofBirthMax=&PartyKind=exact&PartyMax=&list=2022_vlist&ElectionDistrictKind=exact&ElectionDistrictMax=&LegislativeDistrictKind=exact&LegislativeDistrictMax=&TownKind=exact&TownMax=&WardKind=exact&WardMax=&CongressionalDistrictKind=exact&CongressionalDistrictMax=&SenateDistrictKind=exact&SenateDistrictMax=&AssemblyDistrictKind=exact&AssemblyDistrictMax=&LastDateVotedKind=exact&LastDateVotedMax=&LastYearVotedKind=exact&LastYearVotedMax=&LastCountyVotedKind=exact&LastCountyVotedMax=&LastRegisteredAddressKind=exact&LastRegisteredAddressMax=&LastRegisteredNameKind=exact&AssignedCountyKind=exact&AssignedCountyMax=&DateApplicationReceivedKind=exact&DateApplicationReceivedMax=&ApplicationSourceKind=exact&ApplicationSourceMax=&IDRequiredKind=exact&IDRequiredMax=&IDVerifiedKind=exact&IDVerifiedMax=&VoterStatusKind=exact&VoterStatusMax=&StatusReasonKind=exact&StatusReasonMax=&DateInactiveKind=exact&DateInactiveMax=&DatePurgedKind=exact&DatePurgedMax=&IDKind=exact&IDMax=&VoterHistoryKind=exact&VoterHistoryMax=&offset=1&pagesize=50",
            url, args.first_name, args.last_name
        ),
        "NY" => format!(
            "{}?FirstNameKind=contains&FirstNameMax={}&MiddleNameKind=exact&MiddleNameMax=&LastNameKind=starts&LastNameMax={}&NameSuffixKind=exact&NameSuffixMax=&HouseNumberKind=exact&HouseNumberMax=&FractionKind=exact&FractionMax=&ApartmentKind=exact&ApartmentMax=&PreStreetDirectionKind=exact&PreStreetDirectionMax=&streetKind=exact&streetMax=&PostStreetDirectionKind=exact&PostStreetDirectionMax=&CityKind=exact&CityMax=&CountyKind=exact&CountyMax=&Zip5Kind=exact&Zip5Max=&Zip4Kind=exact&Zip4Max=&MailAddress1Kind=exact&MailAddress1Max=&MailAddress2Kind=exact&MailAddress2Max=&MailAddress3Kind=exact&MailAddress3Max=&MailAddress4Kind=exact&MailAddress4Max=&DateofBirthKind=starts&DateofBirthMax=&GenderKind=exact&GenderMax=&PartyKind=exact&PartyMax=&number=9&ElectionDistrictKind=exact&ElectionDistrictMax=&LegislativeDistrictKind=exact&LegislativeDistrictMax=&TownKind=exact&TownMax=&WardKind=exact&WardMax=&CongressionalDistrictKind=exact&CongressionalDistrictMax=&SenateDistrictKind=exact&SenateDistrictMax=&AssemblyDistrictKind=exact&AssemblyDistrictMax=&LastDateVotedKind=exact&LastDateVotedMax=&LastYearVotedKind=exact&LastYearVotedMax=&LastCountyVotedKind=exact&LastCountyVotedMax=&LastRegisteredAddressKind=exact&LastRegisteredAddressMax=&LastRegisteredNameKind=exact&AssignedCountyKind=exact&AssignedCountyMax=&DateApplicationReceivedKind=exact&DateApplicationReceivedMax=&ApplicationSourceKind=exact&ApplicationSourceMax=&IDRequiredKind=exact&IDRequiredMax=&IDVerifiedKind=exact&IDVerifiedMax=&VoterStatusKind=exact&VoterStatusMax=&StatusReasonKind=exact&StatusReasonMax=&DateInactiveKind=exact&DateInactiveMax=&DatePurgedKind=exact&DatePurgedMax=&IDKind=exact&IDMax=&VoterHistoryKind=exact&VoterHistoryMax=&offset=1&pagesize=50",
            url, args.first_name, args.last_name
        ),
        "OH" => format!(
            "{}?firstnameKind=starts&firstnameMax={}&lastnameKind=exact&lastnameMax={}&addressKind=exact&addressMax=&cityKind=exact&cityMax=&stateKind=exact&stateMax=OH&zipKind=starts&zipMax=&dobKind=exact&dobMax=&partyKind=exact&partyMax=&offset=1&pagesize=50",
            url, args.first_name, args.last_name
        ),
        "MI" => format!(
            "{}?firstnameKind=starts&firstnameMax={}&lastnameKind=exact&lastnameMax={}&addressKind=exact&addressMax=&cityKind=exact&cityMax=&stateKind=exact&stateMax=MI&zipKind=starts&zipMax=&dobKind=exact&dobMax=&partyKind=exact&partyMax=&offset=1&pagesize=50",
            url, args.first_name, args.last_name
        ),
        _ => format!(
            "{}?firstnameKind=starts&firstnameMax={}&lastnameKind=exact&lastnameMax={}&addressKind=exact&addressMax=&cityKind=exact&cityMax=&stateKind=exact&stateMax={}&zipKind=starts&zipMax=&dobKind=exact&dobMax=&partyKind=exact&partyMax=&offset=1&pagesize=50",
            url, args.first_name, args.last_name, state_code
        ),
    };

    let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let response = client.get(&url).send().unwrap();
    let body = response.text().unwrap();
    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    if let Some(table_element) = document.select(&table_selector).next() {
        // Extract headers
        let headers: Vec<String> = table_element
            .select(&row_selector)
            .next()
            .unwrap()
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().concat().trim().to_string())
            .collect();

        // Add headers to the table
        table.set_titles(Row::new(headers.iter().map(|h| Cell::new(h)).collect()));

        // Extract and map data to headers
        for row in table_element.select(&row_selector).skip(1) {
            let mut cells: Vec<_> = row.select(&cell_selector).map(|cell| cell.text().collect::<Vec<_>>().concat().trim().to_string()).collect();

            // Handle specific case for Ohio
            if state_code == "OH" {
                if cells.len() > 8 {
                    let county_age = cells[8].clone();
                    let parts: Vec<&str> = county_age.split_whitespace().collect();
                    if parts.len() > 1 {
                        cells[8] = parts[0].to_string();  // County
                        cells.insert(9, parts[1].to_string());  // Age
                    }
                }
            }

            table.add_row(Row::new(cells.iter().map(|c| Cell::new(c)).collect()));
        }
    }

    table.printstd();
}
