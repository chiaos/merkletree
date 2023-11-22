mod common;

mod proof {

	use hex_literal::hex;
	use tiny_merkle::*;

	use tiny_keccak::{Hasher, Keccak};

	fn keccak256(value: &[u8]) -> [u8; 32] {
		let mut hasher = Keccak::v256();
		let mut hash = [0_u8; 32];
		hasher.update(value);
		hasher.finalize(&mut hash);
		hash
	}

	#[derive(Clone, Debug)]
	pub struct KeccakHasher;
	impl tiny_merkle::Hasher for KeccakHasher {
		type Hash = [u8; 32];

		fn hash(&self, value: &[u8]) -> Self::Hash {
			keccak256(value)
		}
	}

	const LARGE_DATA: [&str; 1573] = [
		"a",
		"b",
		"c",
		"d",
		"e",
		"f",
		"g",
		"h",
		"i",
		"j",
		"k",
		"l",
		"m",
		"n",
		"o",
		"p",
		"q",
		"r",
		"s",
		"t",
		"u",
		"v",
		"w",
		"x",
		"y",
		"z",
		"1",
		"2",
		"3",
		"4",
		"5",
		"6",
		"7",
		"8",
		"9",
		"0",
		"!",
		"@",
		"#",
		"$",
		"%",
		"^",
		"&",
		"*",
		"(",
		")",
		"-",
		"_",
		"+",
		"=",
		"[",
		"]",
		"{",
		"}",
		";",
		":",
		"'",
		"<",
		">",
		",",
		".",
		"/",
		"?",
		"|",
		"\\",
		"~",
		"`",
		" ",
		"A",
		"B",
		"C",
		"D",
		"E",
		"F",
		"G",
		"H",
		"I",
		"J",
		"K",
		"L",
		"M",
		"N",
		"O",
		"P",
		"Q",
		"R",
		"S",
		"T",
		"U",
		"V",
		"W",
		"X",
		"Y",
		"Z",
		"Aa",
		"Bb",
		"Cc",
		"Dd",
		"Ee",
		"Ff",
		"Gg",
		"Hh",
		"Ii",
		"Jj",
		"Kk",
		"Ll",
		"Mm",
		"Nn",
		"Oo",
		"Pp",
		"Qq",
		"Rr",
		"Ss",
		"Tt",
		"Uu",
		"Vv",
		"Ww",
		"Xx",
		"Yy",
		"Zz",
		"Aaa",
		"Bbb",
		"Ccc",
		"Ddd",
		"Eee",
		"Fff",
		"Ggg",
		"00000",
		"11111",
		"22222",
		"33333",
		"44444",
		"55555",
		"66666",
		"77777",
		"88888",
		"99999",
		"000000",
		"111111",
		"222222",
		"333333",
		"444444",
		"555555",
		"666666",
		"777777",
		"888888",
		"999999",
		"0000000",
		"1111111",
		"2222222",
		"3333333",
		"4444444",
		"5555555",
		"6666666",
		"7777777",
		"8888888",
		"9999999",
		"00000000",
		"11111111",
		"22222222",
		"33333333",
		"44444444",
		"55555555",
		"66666666",
		"77777777",
		"88888888",
		"99999999",
		"000000000",
		"111111111",
		"222222222",
		"333333333",
		"444444444",
		"555555555",
		"666666666",
		"777777777",
		"888888888",
		"999999999",
		"0000000000",
		"1111111111",
		"2222222222",
		"3333333333",
		"4444444444",
		"5555555555",
		"6666666666",
		"7777777777",
		"8888888888",
		"9999999999",
		"00000000000",
		"11111111111",
		"22222222222",
		"33333333333",
		"44444444444",
		"55555555555",
		"66666666666",
		"77777777777",
		"88888888888",
		"99999999999",
		"000000000000",
		"111111111111",
		"222222222222",
		"333333333333",
		"444444444444",
		"555555555555",
		"666666666666",
		"777777777777",
		"888888888888",
		"999999999999",
		"0000000000000",
		"1111111111111",
		"2222222222222",
		"3333333333333",
		"4444444444444",
		"5555555555555",
		"6666666666666",
		"7777777777777",
		"8888888888888",
		"9999999999999",
		"00000000000000",
		"11111111111111",
		"22222222222222",
		"33333333333333",
		"44444444444444",
		"55555555555555",
		"66666666666666",
		"77777777777777",
		"88888888888888",
		"99999999999999",
		"000000000000000",
		"111111111111111",
		"222222222222222",
		"333333333333333",
		"444444444444444",
		"555555555555555",
		"666666666666666",
		"777777777777777",
		"888888888888888",
		"1001",
		"1002",
		"1003",
		"1004",
		"1005",
		"1006",
		"1007",
		"1008",
		"1009",
		"1010",
		"1011",
		"1012",
		"1013",
		"1014",
		"1015",
		"1016",
		"1017",
		"1018",
		"1019",
		"1020",
		"1021",
		"1022",
		"1023",
		"1024",
		"1025",
		"1026",
		"1027",
		"1028",
		"1029",
		"1030",
		"1031",
		"1032",
		"1033",
		"1034",
		"1035",
		"1036",
		"1037",
		"1038",
		"1039",
		"1040",
		"1041",
		"1042",
		"1043",
		"1044",
		"1045",
		"1046",
		"1047",
		"1048",
		"1049",
		"1050",
		"1051",
		"1052",
		"1053",
		"1054",
		"1055",
		"1056",
		"1057",
		"1058",
		"1059",
		"1060",
		"1061",
		"1062",
		"1063",
		"1064",
		"1065",
		"1066",
		"1067",
		"1068",
		"1069",
		"1070",
		"1071",
		"1072",
		"1073",
		"1074",
		"1075",
		"1076",
		"1077",
		"1078",
		"1079",
		"1080",
		"1081",
		"1082",
		"1083",
		"1084",
		"1085",
		"1086",
		"1087",
		"1088",
		"1089",
		"1090",
		"1091",
		"1092",
		"1093",
		"1094",
		"1095",
		"1096",
		"1097",
		"1098",
		"1099",
		"1100",
		"1101",
		"1102",
		"1103",
		"1104",
		"1105",
		"1106",
		"1107",
		"1108",
		"1109",
		"1110",
		"1111",
		"1112",
		"1113",
		"1114",
		"1115",
		"1116",
		"1117",
		"1118",
		"1119",
		"1120",
		"1121",
		"1122",
		"1123",
		"1124",
		"1125",
		"1126",
		"1127",
		"1128",
		"1129",
		"1130",
		"1131",
		"1132",
		"1133",
		"1134",
		"1135",
		"1136",
		"1137",
		"1138",
		"1139",
		"1140",
		"1141",
		"1142",
		"1143",
		"1144",
		"1145",
		"1146",
		"1147",
		"1148",
		"1149",
		"1150",
		"1151",
		"1152",
		"1153",
		"1154",
		"1155",
		"1156",
		"1157",
		"1158",
		"1159",
		"1160",
		"1161",
		"1162",
		"1163",
		"1164",
		"1165",
		"1166",
		"1167",
		"1168",
		"1169",
		"1170",
		"1171",
		"1172",
		"1173",
		"1174",
		"1175",
		"1176",
		"1177",
		"1178",
		"1179",
		"1180",
		"1181",
		"1182",
		"1183",
		"1184",
		"1185",
		"1186",
		"1187",
		"1188",
		"1189",
		"1190",
		"1191",
		"1192",
		"1193",
		"1194",
		"1195",
		"1196",
		"1197",
		"1198",
		"1199",
		"1200",
		"1201",
		"1202",
		"1203",
		"1204",
		"1205",
		"1206",
		"1207",
		"1208",
		"1209",
		"1210",
		"1211",
		"1212",
		"1213",
		"1214",
		"1215",
		"1216",
		"1217",
		"1218",
		"1219",
		"1220",
		"1221",
		"1222",
		"1223",
		"1224",
		"1225",
		"1226",
		"1227",
		"1228",
		"1229",
		"1230",
		"1231",
		"1232",
		"1233",
		"1234",
		"1235",
		"1236",
		"1237",
		"1238",
		"1239",
		"1240",
		"1241",
		"1242",
		"1243",
		"1244",
		"1245",
		"1246",
		"1247",
		"1248",
		"1249",
		"1250",
		"1251",
		"1252",
		"1253",
		"1254",
		"1255",
		"1256",
		"1257",
		"1258",
		"1259",
		"1260",
		"1261",
		"1262",
		"1263",
		"1264",
		"1265",
		"1266",
		"1267",
		"1268",
		"1269",
		"1270",
		"1271",
		"1272",
		"1273",
		"1274",
		"1275",
		"1276",
		"1277",
		"1278",
		"1279",
		"1280",
		"1281",
		"1282",
		"1283",
		"1284",
		"1285",
		"1286",
		"1287",
		"1288",
		"1289",
		"1290",
		"1291",
		"1292",
		"1293",
		"1294",
		"1295",
		"1296",
		"1297",
		"1298",
		"1299",
		"1300",
		"1301",
		"1302",
		"1303",
		"1304",
		"1305",
		"1306",
		"1307",
		"1308",
		"1309",
		"1310",
		"1311",
		"1312",
		"1313",
		"1314",
		"1315",
		"1316",
		"1317",
		"1318",
		"1319",
		"1320",
		"1321",
		"1322",
		"1323",
		"1324",
		"1325",
		"1326",
		"1327",
		"1328",
		"1329",
		"1330",
		"1331",
		"1332",
		"1333",
		"1334",
		"1335",
		"1336",
		"1337",
		"1338",
		"1339",
		"1340",
		"1341",
		"1342",
		"1343",
		"1344",
		"1345",
		"1346",
		"1347",
		"1348",
		"1349",
		"1350",
		"1351",
		"1352",
		"1353",
		"1354",
		"1355",
		"1356",
		"1357",
		"1358",
		"1359",
		"1360",
		"1361",
		"1362",
		"1363",
		"1364",
		"1365",
		"1366",
		"1367",
		"1368",
		"1369",
		"1370",
		"1371",
		"1372",
		"1373",
		"1374",
		"1375",
		"1376",
		"1377",
		"1378",
		"1379",
		"1380",
		"1381",
		"1382",
		"1383",
		"1384",
		"1385",
		"1386",
		"1387",
		"1388",
		"1389",
		"1390",
		"1391",
		"1392",
		"1393",
		"1394",
		"1395",
		"1396",
		"1397",
		"1398",
		"1399",
		"1400",
		"1401",
		"1402",
		"1403",
		"1404",
		"1405",
		"1406",
		"1407",
		"1408",
		"1409",
		"1410",
		"1411",
		"1412",
		"1413",
		"1414",
		"1415",
		"1416",
		"1417",
		"1418",
		"1419",
		"1420",
		"1421",
		"1422",
		"1423",
		"1424",
		"1425",
		"1426",
		"1427",
		"1428",
		"1429",
		"1430",
		"1431",
		"1432",
		"1433",
		"1434",
		"1435",
		"1436",
		"1437",
		"1438",
		"1439",
		"1440",
		"1441",
		"1442",
		"1443",
		"1444",
		"1445",
		"1446",
		"1447",
		"1448",
		"1449",
		"1450",
		"1451",
		"1452",
		"1453",
		"1454",
		"1455",
		"1456",
		"1457",
		"1458",
		"1459",
		"1460",
		"1461",
		"1462",
		"1463",
		"1464",
		"1465",
		"1466",
		"1467",
		"1468",
		"1469",
		"1470",
		"1471",
		"1472",
		"1473",
		"1474",
		"1475",
		"1476",
		"1477",
		"1478",
		"1479",
		"1480",
		"1481",
		"1482",
		"1483",
		"1484",
		"1485",
		"1486",
		"1487",
		"1488",
		"1489",
		"1490",
		"1491",
		"1492",
		"1493",
		"1494",
		"1495",
		"1496",
		"1497",
		"1498",
		"1499",
		"1500",
		"1501",
		"1502",
		"1503",
		"1504",
		"1505",
		"1506",
		"1507",
		"1508",
		"1509",
		"1510",
		"1511",
		"1512",
		"1513",
		"1514",
		"1515",
		"1516",
		"1517",
		"1518",
		"1519",
		"1520",
		"1521",
		"1522",
		"1523",
		"1524",
		"1525",
		"1526",
		"1527",
		"1528",
		"1529",
		"1530",
		"1531",
		"1532",
		"1533",
		"1534",
		"1535",
		"1536",
		"1537",
		"1538",
		"1539",
		"1540",
		"1541",
		"1542",
		"1543",
		"1544",
		"1545",
		"1546",
		"1547",
		"1548",
		"1549",
		"1550",
		"1551",
		"1552",
		"1553",
		"1554",
		"1555",
		"1556",
		"1557",
		"1558",
		"1559",
		"1560",
		"1561",
		"1562",
		"1563",
		"1564",
		"1565",
		"1566",
		"1567",
		"1568",
		"1569",
		"1570",
		"1571",
		"1572",
		"1573",
		"1574",
		"1575",
		"1576",
		"1577",
		"1578",
		"1579",
		"1580",
		"1581",
		"1582",
		"1583",
		"1584",
		"1585",
		"1586",
		"1587",
		"1588",
		"1589",
		"1590",
		"1591",
		"1592",
		"1593",
		"1594",
		"1595",
		"1596",
		"1597",
		"1598",
		"1599",
		"1600",
		"1601",
		"1602",
		"1603",
		"1604",
		"1605",
		"1606",
		"1607",
		"1608",
		"1609",
		"1610",
		"1611",
		"1612",
		"1613",
		"1614",
		"1615",
		"1616",
		"1617",
		"1618",
		"1619",
		"1620",
		"1621",
		"1622",
		"1623",
		"1624",
		"1625",
		"1626",
		"1627",
		"1628",
		"1629",
		"1630",
		"1631",
		"1632",
		"1633",
		"1634",
		"1635",
		"1636",
		"1637",
		"1638",
		"1639",
		"1640",
		"1641",
		"1642",
		"1643",
		"1644",
		"1645",
		"1646",
		"1647",
		"1648",
		"1649",
		"1650",
		"1651",
		"1652",
		"1653",
		"1654",
		"1655",
		"1656",
		"1657",
		"1658",
		"1659",
		"1660",
		"1661",
		"1662",
		"1663",
		"1664",
		"1665",
		"1666",
		"1667",
		"1668",
		"1669",
		"1670",
		"1671",
		"1672",
		"1673",
		"1674",
		"1675",
		"1676",
		"1677",
		"1678",
		"1679",
		"1680",
		"1681",
		"1682",
		"1683",
		"1684",
		"1685",
		"1686",
		"1687",
		"1688",
		"1689",
		"1690",
		"1691",
		"1692",
		"1693",
		"1694",
		"1695",
		"1696",
		"1697",
		"1698",
		"1699",
		"1700",
		"1701",
		"1702",
		"1703",
		"1704",
		"1705",
		"1706",
		"1707",
		"1708",
		"1709",
		"1710",
		"1711",
		"1712",
		"1713",
		"1714",
		"1715",
		"1716",
		"1717",
		"1718",
		"1719",
		"1720",
		"1721",
		"1722",
		"1723",
		"1724",
		"1725",
		"1726",
		"1727",
		"1728",
		"1729",
		"1730",
		"1731",
		"1732",
		"1733",
		"1734",
		"1735",
		"1736",
		"1737",
		"1738",
		"1739",
		"1740",
		"1741",
		"1742",
		"1743",
		"1744",
		"1745",
		"1746",
		"1747",
		"1748",
		"1749",
		"1750",
		"1751",
		"1752",
		"1753",
		"1754",
		"1755",
		"1756",
		"1757",
		"1758",
		"1759",
		"1760",
		"1761",
		"1762",
		"1763",
		"1764",
		"1765",
		"1766",
		"1767",
		"1768",
		"1769",
		"1770",
		"1771",
		"1772",
		"1773",
		"1774",
		"1775",
		"1776",
		"1777",
		"1778",
		"1779",
		"1780",
		"1781",
		"1782",
		"1783",
		"1784",
		"1785",
		"1786",
		"1787",
		"1788",
		"1789",
		"1790",
		"1791",
		"1792",
		"1793",
		"1794",
		"1795",
		"1796",
		"1797",
		"1798",
		"1799",
		"1800",
		"1801",
		"1802",
		"1803",
		"1804",
		"1805",
		"1806",
		"1807",
		"1808",
		"1809",
		"1810",
		"1811",
		"1812",
		"1813",
		"1814",
		"1815",
		"1816",
		"1817",
		"1818",
		"1819",
		"1820",
		"1821",
		"1822",
		"1823",
		"1824",
		"1825",
		"1826",
		"1827",
		"1828",
		"1829",
		"1830",
		"1831",
		"1832",
		"1833",
		"1834",
		"1835",
		"1836",
		"1837",
		"1838",
		"1839",
		"1840",
		"1841",
		"1842",
		"1843",
		"1844",
		"1845",
		"1846",
		"1847",
		"1848",
		"1849",
		"1850",
		"1851",
		"1852",
		"1853",
		"1854",
		"1855",
		"1856",
		"1857",
		"1858",
		"1859",
		"1860",
		"1861",
		"1862",
		"1863",
		"1864",
		"1865",
		"1866",
		"1867",
		"1868",
		"1869",
		"1870",
		"1871",
		"1872",
		"1873",
		"1874",
		"1875",
		"1876",
		"1877",
		"1878",
		"1879",
		"1880",
		"1881",
		"1882",
		"1883",
		"1884",
		"1885",
		"1886",
		"1887",
		"1888",
		"1889",
		"1890",
		"1891",
		"1892",
		"1893",
		"1894",
		"1895",
		"1896",
		"1897",
		"1898",
		"1899",
		"1900",
		"1901",
		"1902",
		"1903",
		"1904",
		"1905",
		"1906",
		"1907",
		"1908",
		"1909",
		"1910",
		"1911",
		"1912",
		"1913",
		"1914",
		"1915",
		"1916",
		"1917",
		"1918",
		"1919",
		"1920",
		"1921",
		"1922",
		"1923",
		"1924",
		"1925",
		"1926",
		"1927",
		"1928",
		"1929",
		"1930",
		"1931",
		"1932",
		"1933",
		"1934",
		"1935",
		"1936",
		"1937",
		"1938",
		"1939",
		"1940",
		"1941",
		"1942",
		"1943",
		"1944",
		"1945",
		"1946",
		"1947",
		"1948",
		"1949",
		"1950",
		"1951",
		"1952",
		"1953",
		"1954",
		"1955",
		"1956",
		"1957",
		"1958",
		"1959",
		"1960",
		"1961",
		"1962",
		"1963",
		"1964",
		"1965",
		"1966",
		"1967",
		"1968",
		"1969",
		"1970",
		"1971",
		"1972",
		"1973",
		"1974",
		"1975",
		"1976",
		"1977",
		"1978",
		"1979",
		"1980",
		"1981",
		"1982",
		"1983",
		"1984",
		"1985",
		"1986",
		"1987",
		"1988",
		"1989",
		"1990",
		"1991",
		"1992",
		"1993",
		"1994",
		"1995",
		"1996",
		"1997",
		"1998",
		"1999",
		"2000",
		"2001",
		"2002",
		"2003",
		"2004",
		"2005",
		"2006",
		"2007",
		"2008",
		"2009",
		"2010",
		"2011",
		"2012",
		"2013",
		"2014",
		"2015",
		"2016",
		"2017",
		"2018",
		"2019",
		"2020",
		"2021",
		"2022",
		"2023",
		"2024",
		"2025",
		"2026",
		"2027",
		"2028",
		"2029",
		"2030",
		"2031",
		"2032",
		"2033",
		"2034",
		"2035",
		"2036",
		"2037",
		"2038",
		"2039",
		"2040",
		"2041",
		"2042",
		"2043",
		"2044",
		"2045",
		"2046",
		"2047",
		"2048",
		"2049",
		"2050",
		"2051",
		"2052",
		"2053",
		"2054",
		"2055",
		"2056",
		"2057",
		"2058",
		"2059",
		"2060",
		"2061",
		"2062",
		"2063",
		"2064",
		"2065",
		"2066",
		"2067",
		"2068",
		"2069",
		"2070",
		"2071",
		"2072",
		"2073",
		"2074",
		"2075",
		"2076",
		"2077",
		"2078",
		"2079",
		"2080",
		"2081",
		"2082",
		"2083",
		"2084",
		"2085",
		"2086",
		"2087",
		"2088",
		"2089",
		"2090",
		"2091",
		"2092",
		"2093",
		"2094",
		"2095",
		"2096",
		"2097",
		"2098",
		"2099",
		"2100",
		"2101",
		"2102",
		"2103",
		"2104",
		"2105",
		"2106",
		"2107",
		"2108",
		"2109",
		"2110",
		"2111",
		"2112",
		"2113",
		"2114",
		"2115",
		"2116",
		"2117",
		"2118",
		"2119",
		"2120",
		"2121",
		"2122",
		"2123",
		"2124",
		"2125",
		"2126",
		"2127",
		"2128",
		"2129",
		"2130",
		"2131",
		"2132",
		"2133",
		"2134",
		"2135",
		"2136",
		"2137",
		"2138",
		"2139",
		"2140",
		"2141",
		"2142",
		"2143",
		"2144",
		"2145",
		"2146",
		"2147",
		"2148",
		"2149",
		"2150",
		"2151",
		"2152",
		"2153",
		"2154",
		"2155",
		"2156",
		"2157",
		"2158",
		"2159",
		"2160",
		"2161",
		"2162",
		"2163",
		"2164",
		"2165",
		"2166",
		"2167",
		"2168",
		"2169",
		"2170",
		"2171",
		"2172",
		"2173",
		"2174",
		"2175",
		"2176",
		"2177",
		"2178",
		"2179",
		"2180",
		"2181",
		"2182",
		"2183",
		"2184",
		"2185",
		"2186",
		"2187",
		"2188",
		"2189",
		"2190",
		"2191",
		"2192",
		"2193",
		"2194",
		"2195",
		"2196",
		"2197",
		"2198",
		"2199",
		"2200",
		"2201",
		"2202",
		"2203",
		"2204",
		"2205",
		"2206",
		"2207",
		"2208",
		"2209",
		"2210",
		"2211",
		"2212",
		"2213",
		"2214",
		"2215",
		"2216",
		"2217",
		"2218",
		"2219",
		"2220",
		"2221",
		"2222",
		"2223",
		"2224",
		"2225",
		"2226",
		"2227",
		"2228",
		"2229",
		"2230",
		"2231",
		"2232",
		"2233",
		"2234",
		"2235",
		"2236",
		"2237",
		"2238",
		"2239",
		"2240",
		"2241",
		"2242",
		"2243",
		"2244",
		"2245",
		"2246",
		"2247",
		"2248",
		"2249",
		"2250",
		"2251",
		"2252",
		"2253",
		"2254",
		"2255",
		"2256",
		"2257",
		"2258",
		"2259",
		"2260",
		"2261",
		"2262",
		"2263",
		"2264",
		"2265",
		"2266",
		"2267",
		"2268",
		"2269",
		"2270",
		"2271",
		"2272",
		"2273",
		"2274",
		"2275",
		"2276",
		"2277",
		"2278",
		"2279",
		"2280",
		"2281",
		"2282",
		"2283",
		"2284",
		"2285",
		"2286",
		"2287",
		"2288",
		"2289",
		"2290",
		"2291",
		"2292",
		"2293",
		"2294",
		"2295",
		"2296",
		"2297",
		"2298",
		"2299",
		"2300",
		"2301",
		"2302",
		"2303",
		"2304",
		"2305",
		"2306",
		"2307",
		"2308",
		"2309",
		"2310",
		"2311",
		"2312",
		"2313",
		"2314",
		"2315",
		"2316",
		"2317",
		"2318",
		"2319",
		"2320",
		"2321",
		"2322",
		"2323",
		"2324",
		"2325",
		"2326",
		"2327",
		"2328",
		"2329",
		"2330",
		"2331",
		"2332",
		"2333",
		"2334",
		"2335",
		"2336",
		"2337",
	];

	// fn merkle_proof_from_vec<H>(leaf_hashes: Vec<H::Hash>) -> MerkleProof<H>
	//     where H: tiny_merkle::Hasher
	// {

	//     MerkleProof {
	//         data: leaf_hashes,
	//         position: Position::Left,
	//     }
	// }

	#[test]
	fn merkle_root() {
		let expected_root_hex = hex!("17e001de1e46cb5b7856f1ac31728e8005ae908e625c7abcde378a0658c60a0d");
		let leaf_hashes: Vec<_> = LARGE_DATA.iter().map(|x| keccak256(x.as_bytes())).collect();
		let tree = MerkleTree::<KeccakHasher>::new(
			KeccakHasher,
			leaf_hashes.clone(),
			Some(MerkleOptions {
				min_tree_size: None,
				hash_leaves: None,
				sort_leaves: None,
				sort_pairs: None,
				sort: Some(true),
			}),
		);
		assert_eq!(
			tree.root().as_ref(),
			expected_root_hex,
			"root hash mismatch, expected: {:?}, actual: {:?}",
			hex::encode(expected_root_hex),
			hex::encode(tree.root().as_ref())
		);
	}

	macro_rules! test_proof_from_data {
		($name:ident, $src:expr, $proof_exp:expr) => {
			#[test]
			fn $name() {
				let leaf_hashes: Vec<_> = LARGE_DATA.iter().map(|x| keccak256(x.as_bytes())).collect();
				let tree = MerkleTree::<KeccakHasher>::new(
					KeccakHasher,
					leaf_hashes.clone(),
					Some(MerkleOptions {
						min_tree_size: None,
						hash_leaves: None,
						sort_leaves: None,
						sort_pairs: None,
						sort: Some(true),
					}),
				);

				let proof = tree.proof(keccak256($src.as_bytes()).as_ref()).expect("Failed to generate proof");
				let p = proof.iter().map(|p| format!("0x{}", hex::encode(p.data))).collect::<Vec<_>>();
				assert_eq!(p, $proof_exp, "proof mismatch, expected: {:?}, actual: {:?}", $proof_exp, p);
			}
		};

		($name:ident, $src:expr, $proof_exp:expr,$pan:expr) => {
			#[test]
			#[should_panic(expected = $pan)]
			fn $name() {
				let leaf_hashes: Vec<_> = LARGE_DATA.iter().map(|x| keccak256(x.as_bytes())).collect();
				let tree = MerkleTree::<KeccakHasher>::new(
					KeccakHasher,
					leaf_hashes.clone(),
					Some(MerkleOptions {
						min_tree_size: None,
						hash_leaves: None,
						sort_leaves: None,
						sort_pairs: None,
						sort: Some(true),
					}),
				);

				let proof = tree.proof(keccak256($src.as_bytes()).as_ref()).expect("Failed to generate proof");
				let p = proof.iter().map(|p| format!("0x{}", hex::encode(p.data))).collect::<Vec<_>>();
				assert_eq!(p, $proof_exp, "proof mismatch, expected: {:?}, actual: {:?}", $proof_exp, p);
			}
		};
	}

	test_proof_from_data!(should_panic_on_non_existing_leaf, "non-existing", [""], "Failed to generate proof");

	test_proof_from_data!(
		case_a,
		"a",
		[
			"0x3aed78414b1d4308dcf55364fe60a7b4c60d3a5d1da8bf39303779f802c24453",
			"0x6bfa6eff624e38741b3e15279e9acf79d17533c67d0f6507b1503840f894489a",
			"0x98b675f05f7542d3236f4dcb45b6997e884d595ce46fd66832432d6d3987147e",
			"0x893ff2e0af6a29c2982a1070a9956de9adfbc4715334a7863fdfe8e03ad3df14",
			"0xca47dfd8742595b9d5b93883a963134308db3775138ebba116486b7db1e1535f",
			"0xbea7cb77c67feb68d6d9ae5a090a7015c179bb9844fdc0faa005b3d0542a12e1",
			"0x5be122ccf911b85197bbc993048ec4ed808c80aa452e536fe7acdae5df7f6caa",
			"0xc77d1197b4be24ce54c9ae618c4f536781785b148f21b4c563fba0f315f49c8a",
			"0x69f108c61827d6d5fa40b8ee7a474157d426483773793571e3f014ac5b4039fc",
			"0x6284fe654bc111ef669d46c4defb300e4add87cc3fe40059b74b1d1c37b4b0ae",
			"0x421df41eca3b9f77e8c1eabac99d193084f9343a334d44d58fe20f84b1e53cb4"
		]
	);

	test_proof_from_data!(
		case_2011,
		"2011",
		[
			"0x82406189ee4c2aa1bb703db882a60aded39c441f7b440f5760a857f638d93571",
			"0x266f581c612271f9e1435c10b2de7064dca365d85ae80973f2b13b214117ac3a",
			"0x425f98f00cddb1c2dbfe0a03468f76633f0450ebdcf8202c1973b2d2602e9e38",
			"0x9d699d1685f29d921f2944af5334ddcdd13fdd4429de39ae7d0f08558789de6f",
			"0x81643a8a6a990ff64748edff29d2a44bd833a3a00c052621c415cb5a7eafb785",
			"0x1b3a2aa88e8f4382b1aa0a67b368cfbcd5f38e47d75b54a17d9d2851978ef9a1",
			"0x489ef136a3b0321c960b7923c317ee20586b51c142124ddb8ecde6f46a4d98bd",
			"0x680444ab146854456ef72de411ba01ca1587a378114ede061635b233d79346e1",
			"0x4db313fc9fbbb098a45350e49a6b658089d30daf4fd9bad2978fc80520da1216",
			"0x1f9fca43bdaf9dd271d4eb656d70d3aa1ec30cdb90fd7aa73895729aac819d34",
			"0x421df41eca3b9f77e8c1eabac99d193084f9343a334d44d58fe20f84b1e53cb4"
		]
	);

	test_proof_from_data!(
		case_99999,
		"99999",
		[
			"0x9b201ac9a40a788f048a98b85eb58e7b1af47012ad266ac72c37a39fe14b4e5f",
			"0xac3273f0b7e53ec330763cfffddc256524ac13a2f017dae79be335788de1a924",
			"0x7c769b6ba58592e2b5ec45019c10cd81c2fcc196f48ecf6a84c23a2238fb8582",
			"0xadc604011bcfd6db15c74e4d5cb93e408b30e3981703fb7eeaa7ff38ee3ec771",
			"0xed32b7b810c55287dc94cdefbd6840b818f34c333dfb834a59efb1bd3a3424ee",
			"0x50712f0a3ebd333ed53ba7505250d33027c159723a8681e0611b580881603f64",
			"0x19e9bcbb2c3b11ff47c1953b31c1dd54c46beda85719fcfd1cd7cb25ea72d5c4",
			"0x79702f68a11d14cbe6aef2287a78e50a88f1344c68cf389ee1431ae5b6952e76",
			"0x4db313fc9fbbb098a45350e49a6b658089d30daf4fd9bad2978fc80520da1216",
			"0x1f9fca43bdaf9dd271d4eb656d70d3aa1ec30cdb90fd7aa73895729aac819d34",
			"0x421df41eca3b9f77e8c1eabac99d193084f9343a334d44d58fe20f84b1e53cb4"
		]
	);

	test_proof_from_data!(
		case_div,
		"\\",
		[
			"0x72fc7ab3e40b1907e59758f569afe04bc56a27641f6a4e6ae11ca369e7d44bde",
			"0x01213d608caec3f0e9c40cfe6b8017549530cc3f7e00c617d1f56d75868e99ff",
			"0xb69dce70f35bcf0fd4f564b8bbe32e1249146b004f5dd03aba79d3f33ccd8f5d",
			"0x3431cbda1f0774b3a2d2ad1d50b5238b5e9175f97e2662ad10b37fccf7402e88",
			"0xed380171e18d1b602ee285d4ea11d10d7c971c37e489dcda3d46f4525093b3b8",
			"0xaa75ca277c224253187432e6c5c9622dad3c9f82c7b3ff19122461c80eb1923c",
			"0x5247226d3a152e6411d6e0fdcfee9c0ada2c87f80f667350521669601aa4b2d1",
			"0x081b432e8897c907b438c83ee32b35c141f12358a34fc0a6312118ddbcd130ac",
			"0x8cf2aff741dc53147a58f3a32b302f0603924526bddbb5561c882bc5cc7e65fa",
			"0x1f9fca43bdaf9dd271d4eb656d70d3aa1ec30cdb90fd7aa73895729aac819d34",
			"0x421df41eca3b9f77e8c1eabac99d193084f9343a334d44d58fe20f84b1e53cb4"
		]
	);

	#[test]
	#[ignore]
	fn keccak256_proof() {
		let leaf_hashes: Vec<_> = LARGE_DATA.iter().map(|x| keccak256(x.as_bytes())).collect();
		let tree = MerkleTree::<KeccakHasher>::new(
			KeccakHasher,
			leaf_hashes.clone(),
			Some(MerkleOptions {
				min_tree_size: None,
				hash_leaves: None,
				sort_leaves: None,
				sort_pairs: None,
				sort: Some(true),
			}),
		);

		for (i, leaf) in leaf_hashes.iter().enumerate() {
			let proof = tree.proof(leaf).expect("Failed to generate proof");
			assert!(
				tree.verify(leaf, tree.root().as_ref(), &proof),
				"Failed to verify proof, leaf: {:?}, value: {:?}",
				hex::encode(leaf),
				LARGE_DATA[i]
			);
		}
	}
}
