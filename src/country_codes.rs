use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
pub enum Country {
	#[strum(serialize = "AF", serialize = "AFG", serialize = "4")]
	Afghanistan,
	#[strum(serialize = "AX", serialize = "ALA", serialize = "248")]
	AlandIslands,
	#[strum(serialize = "AL", serialize = "ALB", serialize = "8")]
	Albania,
	#[strum(serialize = "DZ", serialize = "DZA", serialize = "12")]
	Algeria,
	#[strum(serialize = "AS", serialize = "ASM", serialize = "16")]
	AmericanSamoa,
	#[strum(serialize = "AD", serialize = "AND", serialize = "20")]
	Andorra,
	#[strum(serialize = "AO", serialize = "AGO", serialize = "24")]
	Angola,
	#[strum(serialize = "AI", serialize = "AIA", serialize = "660")]
	Anguilla,
	#[strum(serialize = "AQ", serialize = "ATA", serialize = "10")]
	Antarctica,
	#[strum(serialize = "AG", serialize = "ATG", serialize = "28")]
	AntiguaAndBarbuda,
	#[strum(serialize = "AR", serialize = "ARG", serialize = "32")]
	Argentina,
	#[strum(serialize = "AM", serialize = "ARM", serialize = "51")]
	Armenia,
	#[strum(serialize = "AW", serialize = "ABW", serialize = "533")]
	Aruba,
	#[strum(serialize = "AU", serialize = "AUS", serialize = "36")]
	Australia,
	#[strum(serialize = "AT", serialize = "AUT", serialize = "40")]
	Austria,
	#[strum(serialize = "AZ", serialize = "AZE", serialize = "31")]
	Azerbaijan,
	#[strum(serialize = "BS", serialize = "BHS", serialize = "44")]
	Bahamas,
	#[strum(serialize = "BH", serialize = "BHR", serialize = "48")]
	Bahrain,
	#[strum(serialize = "BD", serialize = "BGD", serialize = "50")]
	Bangladesh,
	#[strum(serialize = "BB", serialize = "BRB", serialize = "52")]
	Barbados,
	#[strum(serialize = "BY", serialize = "BLR", serialize = "112")]
	Belarus,
	#[strum(serialize = "BE", serialize = "BEL", serialize = "56")]
	Belgium,
	#[strum(serialize = "BZ", serialize = "BLZ", serialize = "84")]
	Belize,
	#[strum(serialize = "BJ", serialize = "BEN", serialize = "204")]
	Benin,
	#[strum(serialize = "BM", serialize = "BMU", serialize = "60")]
	Bermuda,
	#[strum(serialize = "BT", serialize = "BTN", serialize = "64")]
	Bhutan,
	#[strum(serialize = "BO", serialize = "BOL", serialize = "68")]
	Bolivia,
	#[strum(serialize = "BQ", serialize = "BES", serialize = "535")]
	BonaireSintEustatiusAndSaba,
	#[strum(serialize = "BA", serialize = "BIH", serialize = "70")]
	BosniaAndHerzegovina,
	#[strum(serialize = "BW", serialize = "BWA", serialize = "72")]
	Botswana,
	#[strum(serialize = "BV", serialize = "BVT", serialize = "74")]
	BouvetIsland,
	#[strum(serialize = "BR", serialize = "BRA", serialize = "76")]
	Brazil,
	#[strum(serialize = "IO", serialize = "IOT", serialize = "86")]
	BritishIndianOceanTerritory,
	#[strum(serialize = "BN", serialize = "BRN", serialize = "96")]
	BruneiDarussalam,
	#[strum(serialize = "BG", serialize = "BGR", serialize = "100")]
	Bulgaria,
	#[strum(serialize = "BF", serialize = "BFA", serialize = "854")]
	BurkinaFaso,
	#[strum(serialize = "BI", serialize = "BDI", serialize = "108")]
	Burundi,
	#[strum(serialize = "CV", serialize = "CPV", serialize = "132")]
	CaboVerde,
	#[strum(serialize = "KH", serialize = "KHM", serialize = "116")]
	Cambodia,
	#[strum(serialize = "CM", serialize = "CMR", serialize = "120")]
	Cameroon,
	#[strum(serialize = "CA", serialize = "CAN", serialize = "124")]
	Canada,
	#[strum(serialize = "KY", serialize = "CYM", serialize = "136")]
	CaymanIslands,
	#[strum(serialize = "CF", serialize = "CAF", serialize = "140")]
	CentralAfricanRepublic,
	#[strum(serialize = "TD", serialize = "TCD", serialize = "148")]
	Chad,
	#[strum(serialize = "CL", serialize = "CHL", serialize = "152")]
	Chile,
	#[strum(serialize = "CN", serialize = "CHN", serialize = "156")]
	China,
	#[strum(serialize = "CX", serialize = "CXR", serialize = "162")]
	ChristmasIsland,
	#[strum(serialize = "CC", serialize = "CCK", serialize = "166")]
	Cocos,
	#[strum(serialize = "CO", serialize = "COL", serialize = "170")]
	Colombia,
	#[strum(serialize = "KM", serialize = "COM", serialize = "174")]
	Comoros,
	#[strum(serialize = "CD", serialize = "COD", serialize = "180")]
	DemocraticRepublicOfTheCongo,
	#[strum(serialize = "CG", serialize = "COG", serialize = "178")]
	Congo,
	#[strum(serialize = "CK", serialize = "COK", serialize = "184")]
	CookIslands,
	#[strum(serialize = "CR", serialize = "CRI", serialize = "188")]
	CostaRica,
	#[strum(serialize = "CI", serialize = "CIV", serialize = "384")]
	CoteDIvoire,
	#[strum(serialize = "HR", serialize = "HRV", serialize = "191")]
	Croatia,
	#[strum(serialize = "CU", serialize = "CUB", serialize = "192")]
	Cuba,
	#[strum(serialize = "CW", serialize = "CUW", serialize = "531")]
	Curaçao,
	#[strum(serialize = "CY", serialize = "CYP", serialize = "196")]
	Cyprus,
	#[strum(serialize = "CZ", serialize = "CZE", serialize = "203")]
	Czechia,
	#[strum(serialize = "DK", serialize = "DNK", serialize = "208")]
	Denmark,
	#[strum(serialize = "DJ", serialize = "DJI", serialize = "262")]
	Djibouti,
	#[strum(serialize = "DM", serialize = "DMA", serialize = "212")]
	Dominica,
	#[strum(serialize = "DO", serialize = "DOM", serialize = "214")]
	DominicanRepublic,
	#[strum(serialize = "EC", serialize = "ECU", serialize = "218")]
	Ecuador,
	#[strum(serialize = "EG", serialize = "EGY", serialize = "818")]
	Egypt,
	#[strum(serialize = "SV", serialize = "SLV", serialize = "222")]
	ElSalvador,
	#[strum(serialize = "GQ", serialize = "GNQ", serialize = "226")]
	EquatorialGuinea,
	#[strum(serialize = "ER", serialize = "ERI", serialize = "232")]
	Eritrea,
	#[strum(serialize = "EE", serialize = "EST", serialize = "233")]
	Estonia,
	#[strum(serialize = "SZ", serialize = "SWZ", serialize = "748")]
	Eswatini,
	#[strum(serialize = "ET", serialize = "ETH", serialize = "231")]
	Ethiopia,
	#[strum(serialize = "FK", serialize = "FLK", serialize = "238")]
	FalklandIslands,
	#[strum(serialize = "FO", serialize = "FRO", serialize = "234")]
	FaroeIslands,
	#[strum(serialize = "FJ", serialize = "FJI", serialize = "242")]
	Fiji,
	#[strum(serialize = "FI", serialize = "FIN", serialize = "246")]
	Finland,
	#[strum(serialize = "FR", serialize = "FRA", serialize = "250")]
	France,
	#[strum(serialize = "GF", serialize = "GUF", serialize = "254")]
	FrenchGuiana,
	#[strum(serialize = "PF", serialize = "PYF", serialize = "258")]
	FrenchPolynesia,
	#[strum(serialize = "TF", serialize = "ATF", serialize = "260")]
	FrenchSouthernTerritories,
	#[strum(serialize = "GA", serialize = "GAB", serialize = "266")]
	Gabon,
	#[strum(serialize = "GM", serialize = "GMB", serialize = "270")]
	Gambia,
	#[strum(serialize = "GE", serialize = "GEO", serialize = "268")]
	Georgia,
	#[strum(serialize = "DE", serialize = "DEU", serialize = "276")]
	Germany,
	#[strum(serialize = "GH", serialize = "GHA", serialize = "288")]
	Ghana,
	#[strum(serialize = "GI", serialize = "GIB", serialize = "292")]
	Gibraltar,
	#[strum(serialize = "GR", serialize = "GRC", serialize = "300")]
	Greece,
	#[strum(serialize = "GL", serialize = "GRL", serialize = "304")]
	Greenland,
	#[strum(serialize = "GD", serialize = "GRD", serialize = "308")]
	Grenada,
	#[strum(serialize = "GP", serialize = "GLP", serialize = "312")]
	Guadeloupe,
	#[strum(serialize = "GU", serialize = "GUM", serialize = "316")]
	Guam,
	#[strum(serialize = "GT", serialize = "GTM", serialize = "320")]
	Guatemala,
	#[strum(serialize = "GG", serialize = "GGY", serialize = "831")]
	Guernsey,
	#[strum(serialize = "GN", serialize = "GIN", serialize = "324")]
	Guinea,
	#[strum(serialize = "GW", serialize = "GNB", serialize = "624")]
	GuineaBissau,
	#[strum(serialize = "GY", serialize = "GUY", serialize = "328")]
	Guyana,
	#[strum(serialize = "HT", serialize = "HTI", serialize = "332")]
	Haiti,
	#[strum(serialize = "HM", serialize = "HMD", serialize = "334")]
	HeardIslandAndMcdonaldIslands,
	#[strum(serialize = "VA", serialize = "VAT", serialize = "336")]
	HolySee,
	#[strum(serialize = "HN", serialize = "HND", serialize = "340")]
	Honduras,
	#[strum(serialize = "HK", serialize = "HKG", serialize = "344")]
	HongKong,
	#[strum(serialize = "HU", serialize = "HUN", serialize = "348")]
	Hungary,
	#[strum(serialize = "IS", serialize = "ISL", serialize = "352")]
	Iceland,
	#[strum(serialize = "IN", serialize = "IND", serialize = "356")]
	India,
	#[strum(serialize = "ID", serialize = "IDN", serialize = "360")]
	Indonesia,
	#[strum(serialize = "IR", serialize = "IRN", serialize = "364")]
	Iran,
	#[strum(serialize = "IQ", serialize = "IRQ", serialize = "368")]
	Iraq,
	#[strum(serialize = "IE", serialize = "IRL", serialize = "372")]
	Ireland,
	#[strum(serialize = "IM", serialize = "IMN", serialize = "833")]
	IsleOfMan,
	#[strum(serialize = "IL", serialize = "ISR", serialize = "376")]
	Israel,
	#[strum(serialize = "IT", serialize = "ITA", serialize = "380")]
	Italy,
	#[strum(serialize = "JM", serialize = "JAM", serialize = "388")]
	Jamaica,
	#[strum(serialize = "JP", serialize = "JPN", serialize = "392")]
	Japan,
	#[strum(serialize = "JE", serialize = "JEY", serialize = "832")]
	Jersey,
	#[strum(serialize = "JO", serialize = "JOR", serialize = "400")]
	Jordan,
	#[strum(serialize = "KZ", serialize = "KAZ", serialize = "398")]
	Kazakhstan,
	#[strum(serialize = "KE", serialize = "KEN", serialize = "404")]
	Kenya,
	#[strum(serialize = "KI", serialize = "KIR", serialize = "296")]
	Kiribati,
	#[strum(serialize = "KP", serialize = "PRK", serialize = "408")]
	DemocraticPeopleSRepublicOfKorea,
	#[strum(serialize = "KR", serialize = "KOR", serialize = "410")]
	Korea,
	#[strum(serialize = "KW", serialize = "KWT", serialize = "414")]
	Kuwait,
	#[strum(serialize = "KG", serialize = "KGZ", serialize = "417")]
	Kyrgyzstan,
	#[strum(serialize = "LA", serialize = "LAO", serialize = "418")]
	LaoPeopleSDemocraticRepublic,
	#[strum(serialize = "LV", serialize = "LVA", serialize = "428")]
	Latvia,
	#[strum(serialize = "LB", serialize = "LBN", serialize = "422")]
	Lebanon,
	#[strum(serialize = "LS", serialize = "LSO", serialize = "426")]
	Lesotho,
	#[strum(serialize = "LR", serialize = "LBR", serialize = "430")]
	Liberia,
	#[strum(serialize = "LY", serialize = "LBY", serialize = "434")]
	Libya,
	#[strum(serialize = "LI", serialize = "LIE", serialize = "438")]
	Liechtenstein,
	#[strum(serialize = "LT", serialize = "LTU", serialize = "440")]
	Lithuania,
	#[strum(serialize = "LU", serialize = "LUX", serialize = "442")]
	Luxembourg,
	#[strum(serialize = "MO", serialize = "MAC", serialize = "446")]
	Macao,
	#[strum(serialize = "MK", serialize = "MKD", serialize = "807")]
	RepublicOfNorthMacedonia,
	#[strum(serialize = "MG", serialize = "MDG", serialize = "450")]
	Madagascar,
	#[strum(serialize = "MW", serialize = "MWI", serialize = "454")]
	Malawi,
	#[strum(serialize = "MY", serialize = "MYS", serialize = "458")]
	Malaysia,
	#[strum(serialize = "MV", serialize = "MDV", serialize = "462")]
	Maldives,
	#[strum(serialize = "ML", serialize = "MLI", serialize = "466")]
	Mali,
	#[strum(serialize = "MT", serialize = "MLT", serialize = "470")]
	Malta,
	#[strum(serialize = "MH", serialize = "MHL", serialize = "584")]
	MarshallIslands,
	#[strum(serialize = "MQ", serialize = "MTQ", serialize = "474")]
	Martinique,
	#[strum(serialize = "MR", serialize = "MRT", serialize = "478")]
	Mauritania,
	#[strum(serialize = "MU", serialize = "MUS", serialize = "480")]
	Mauritius,
	#[strum(serialize = "YT", serialize = "MYT", serialize = "175")]
	Mayotte,
	#[strum(serialize = "MX", serialize = "MEX", serialize = "484")]
	Mexico,
	#[strum(serialize = "FM", serialize = "FSM", serialize = "583")]
	Micronesia,
	#[strum(serialize = "MD", serialize = "MDA", serialize = "498")]
	Moldova,
	#[strum(serialize = "MC", serialize = "MCO", serialize = "492")]
	Monaco,
	#[strum(serialize = "MN", serialize = "MNG", serialize = "496")]
	Mongolia,
	#[strum(serialize = "ME", serialize = "MNE", serialize = "499")]
	Montenegro,
	#[strum(serialize = "MS", serialize = "MSR", serialize = "500")]
	Montserrat,
	#[strum(serialize = "MA", serialize = "MAR", serialize = "504")]
	Morocco,
	#[strum(serialize = "MZ", serialize = "MOZ", serialize = "508")]
	Mozambique,
	#[strum(serialize = "MM", serialize = "MMR", serialize = "104")]
	Myanmar,
	#[strum(serialize = "NA", serialize = "NAM", serialize = "516")]
	Namibia,
	#[strum(serialize = "NR", serialize = "NRU", serialize = "520")]
	Nauru,
	#[strum(serialize = "NP", serialize = "NPL", serialize = "524")]
	Nepal,
	#[strum(serialize = "NL", serialize = "NLD", serialize = "528")]
	Netherlands,
	#[strum(serialize = "NC", serialize = "NCL", serialize = "540")]
	NewCaledonia,
	#[strum(serialize = "NZ", serialize = "NZL", serialize = "554")]
	NewZealand,
	#[strum(serialize = "NI", serialize = "NIC", serialize = "558")]
	Nicaragua,
	#[strum(serialize = "NE", serialize = "NER", serialize = "562")]
	Niger,
	#[strum(serialize = "NG", serialize = "NGA", serialize = "566")]
	Nigeria,
	#[strum(serialize = "NU", serialize = "NIU", serialize = "570")]
	Niue,
	#[strum(serialize = "NF", serialize = "NFK", serialize = "574")]
	NorfolkIsland,
	#[strum(serialize = "MP", serialize = "MNP", serialize = "580")]
	NorthernMarianaIslands,
	#[strum(serialize = "NO", serialize = "NOR", serialize = "578")]
	Norway,
	#[strum(serialize = "OM", serialize = "OMN", serialize = "512")]
	Oman,
	#[strum(serialize = "PK", serialize = "PAK", serialize = "586")]
	Pakistan,
	#[strum(serialize = "PW", serialize = "PLW", serialize = "585")]
	Palau,
	#[strum(serialize = "PS", serialize = "PSE", serialize = "275")]
	PalestineStateOf,
	#[strum(serialize = "PA", serialize = "PAN", serialize = "591")]
	Panama,
	#[strum(serialize = "PG", serialize = "PNG", serialize = "598")]
	PapuaNewGuinea,
	#[strum(serialize = "PY", serialize = "PRY", serialize = "600")]
	Paraguay,
	#[strum(serialize = "PE", serialize = "PER", serialize = "604")]
	Peru,
	#[strum(serialize = "PH", serialize = "PHL", serialize = "608")]
	Philippines,
	#[strum(serialize = "PN", serialize = "PCN", serialize = "612")]
	Pitcairn,
	#[strum(serialize = "PL", serialize = "POL", serialize = "616")]
	Poland,
	#[strum(serialize = "PT", serialize = "PRT", serialize = "620")]
	Portugal,
	#[strum(serialize = "PR", serialize = "PRI", serialize = "630")]
	PuertoRico,
	#[strum(serialize = "QA", serialize = "QAT", serialize = "634")]
	Qatar,
	#[strum(serialize = "RE", serialize = "REU", serialize = "638")]
	Reunion,
	#[strum(serialize = "RO", serialize = "ROU", serialize = "642")]
	Romania,
	#[strum(serialize = "RU", serialize = "RUS", serialize = "643")]
	RussianFederation,
	#[strum(serialize = "RW", serialize = "RWA", serialize = "646")]
	Rwanda,
	#[strum(serialize = "BL", serialize = "BLM", serialize = "652")]
	SaintBarthelemy,
	#[strum(serialize = "SH", serialize = "SHN", serialize = "654")]
	SaintHelenaAscensionAndTristanDaCunha,
	#[strum(serialize = "KN", serialize = "KNA", serialize = "659")]
	SaintKittsAndNevis,
	#[strum(serialize = "LC", serialize = "LCA", serialize = "662")]
	SaintLucia,
	#[strum(serialize = "MF", serialize = "MAF", serialize = "663")]
	SaintMartin,
	#[strum(serialize = "PM", serialize = "SPM", serialize = "666")]
	SaintPierreAndMiquelon,
	#[strum(serialize = "VC", serialize = "VCT", serialize = "670")]
	SaintVincentAndTheGrenadines,
	#[strum(serialize = "WS", serialize = "WSM", serialize = "882")]
	Samoa,
	#[strum(serialize = "SM", serialize = "SMR", serialize = "674")]
	SanMarino,
	#[strum(serialize = "ST", serialize = "STP", serialize = "678")]
	SaoTomeAndPrincipe,
	#[strum(serialize = "SA", serialize = "SAU", serialize = "682")]
	SaudiArabia,
	#[strum(serialize = "SN", serialize = "SEN", serialize = "686")]
	Senegal,
	#[strum(serialize = "RS", serialize = "SRB", serialize = "688")]
	Serbia,
	#[strum(serialize = "SC", serialize = "SYC", serialize = "690")]
	Seychelles,
	#[strum(serialize = "SL", serialize = "SLE", serialize = "694")]
	SierraLeone,
	#[strum(serialize = "SG", serialize = "SGP", serialize = "702")]
	Singapore,
	#[strum(serialize = "SX", serialize = "SXM", serialize = "534")]
	SintMaarten,
	#[strum(serialize = "SK", serialize = "SVK", serialize = "703")]
	Slovakia,
	#[strum(serialize = "SI", serialize = "SVN", serialize = "705")]
	Slovenia,
	#[strum(serialize = "SB", serialize = "SLB", serialize = "90")]
	SolomonIslands,
	#[strum(serialize = "SO", serialize = "SOM", serialize = "706")]
	Somalia,
	#[strum(serialize = "ZA", serialize = "ZAF", serialize = "710")]
	SouthAfrica,
	#[strum(serialize = "GS", serialize = "SGS", serialize = "239")]
	SouthGeorgiaAndTheSouthSandwichIslands,
	#[strum(serialize = "SS", serialize = "SSD", serialize = "728")]
	SouthSudan,
	#[strum(serialize = "ES", serialize = "ESP", serialize = "724")]
	Spain,
	#[strum(serialize = "LK", serialize = "LKA", serialize = "144")]
	SriLanka,
	#[strum(serialize = "SD", serialize = "SDN", serialize = "729")]
	Sudan,
	#[strum(serialize = "SR", serialize = "SUR", serialize = "740")]
	Suriname,
	#[strum(serialize = "SJ", serialize = "SJM", serialize = "744")]
	SvalbardAndJanMayen,
	#[strum(serialize = "SE", serialize = "SWE", serialize = "752")]
	Sweden,
	#[strum(serialize = "CH", serialize = "CHE", serialize = "756")]
	Switzerland,
	#[strum(serialize = "SY", serialize = "SYR", serialize = "760")]
	SyrianArabRepublic,
	#[strum(serialize = "TW", serialize = "TWN", serialize = "158")]
	Taiwan,
	#[strum(serialize = "TJ", serialize = "TJK", serialize = "762")]
	Tajikistan,
	#[strum(serialize = "TZ", serialize = "TZA", serialize = "834")]
	TanzaniaUnitedRepublicOf,
	#[strum(serialize = "TH", serialize = "THA", serialize = "764")]
	Thailand,
	#[strum(serialize = "TL", serialize = "TLS", serialize = "626")]
	TimorLeste,
	#[strum(serialize = "TG", serialize = "TGO", serialize = "768")]
	Togo,
	#[strum(serialize = "TK", serialize = "TKL", serialize = "772")]
	Tokelau,
	#[strum(serialize = "TO", serialize = "TON", serialize = "776")]
	Tonga,
	#[strum(serialize = "TT", serialize = "TTO", serialize = "780")]
	TrinidadAndTobago,
	#[strum(serialize = "TN", serialize = "TUN", serialize = "788")]
	Tunisia,
	#[strum(serialize = "TR", serialize = "TUR", serialize = "792")]
	Turkey,
	#[strum(serialize = "TM", serialize = "TKM", serialize = "795")]
	Turkmenistan,
	#[strum(serialize = "TC", serialize = "TCA", serialize = "796")]
	TurksAndCaicosIslands,
	#[strum(serialize = "TV", serialize = "TUV", serialize = "798")]
	Tuvalu,
	#[strum(serialize = "UG", serialize = "UGA", serialize = "800")]
	Uganda,
	#[strum(serialize = "UA", serialize = "UKR", serialize = "804")]
	Ukraine,
	#[strum(serialize = "AE", serialize = "ARE", serialize = "784")]
	UnitedArabEmirates,
	#[strum(serialize = "GB", serialize = "GBR", serialize = "826")]
	UnitedKingdomOfGreatBritainAndNorthernIreland,
	#[strum(serialize = "UM", serialize = "UMI", serialize = "581")]
	UnitedStatesMinorOutlyingIslands,
	#[strum(serialize = "US", serialize = "USA", serialize = "840")]
	UnitedStatesOfAmerica,
	#[strum(serialize = "UY", serialize = "URY", serialize = "858")]
	Uruguay,
	#[strum(serialize = "UZ", serialize = "UZB", serialize = "860")]
	Uzbekistan,
	#[strum(serialize = "VU", serialize = "VUT", serialize = "548")]
	Vanuatu,
	#[strum(serialize = "VE", serialize = "VEN", serialize = "862")]
	Venezuela,
	#[strum(serialize = "VN", serialize = "VNM", serialize = "704")]
	VietNam,
	#[strum(serialize = "VG", serialize = "VGB", serialize = "92")]
	BritishVirginIslands,
	#[strum(serialize = "VI", serialize = "VIR", serialize = "850")]
	VirginIslands,
	#[strum(serialize = "WF", serialize = "WLF", serialize = "876")]
	WallisAndFutuna,
	#[strum(serialize = "EH", serialize = "ESH", serialize = "732")]
	WesternSahara,
	#[strum(serialize = "YE", serialize = "YEM", serialize = "887")]
	Yemen,
	#[strum(serialize = "ZM", serialize = "ZMB", serialize = "894")]
	Zambia,
	#[strum(serialize = "ZW", serialize = "ZWE", serialize = "716")]
	Zimbabwe,
}
