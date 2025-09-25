package main

import (
	"database/sql"
	"encoding/xml"
	"fmt"
	"io"
	"net/http"
	"os"

	_ "github.com/mattn/go-sqlite3"
)

type TrainOperatingCompanyList struct {
	XMLName      xml.Name       `xml:"TrainOperatingCompanyList"`
	TrainCompany []TrainCompany `xml:"TrainOperatingCompany"`
}

type TrainCompany struct {
	XMLName         xml.Name `xml:"TrainOperatingCompany"`
	Name            string   `xml:"Name"`
	AtocCode        string   `xml:"AtocCode"`
	StationOperator bool     `xml:"StationOperator"`
	Logo            string   `xml:"Logo"`
	LegalName       string   `xml:"LegalName"`
}

type StationList struct {
	XMLName xml.Name  `xml:"StationList"`
	Station []Station `xml:"Station"`
}

type Station struct {
	XMLName                xml.Name                      `xml:"Station"`
	Name                   string                        `xml:"Name"`
	Longitude              string                        `xml:"Longitude"`
	Latitude               string                        `xml:"Latitude"`
	Code                   string                        `xml:"CrsCode"`
	AlternativeIdentifiers StationAlternativeIdentifiers `xml:"AlternativeIdentifiers"`
}

type StationAlternativeIdentifiers struct {
	XMLName              xml.Name `xml:"AlternativeIdentifiers"`
	NationalLocationCode string   `xml:"NationalLocationCode"`
}

func main() {
	var toc_api_key = os.Getenv("NR_TOC_API_KEY")
	var station_api_key = os.Getenv("NR_STATION_API_KEY")
	var db_url = os.Getenv("DB_URL")
	println("TOC API KEY: " + toc_api_key)
	println("STATION API KEY: ", station_api_key)
	println("DB_URL: ", db_url)
	db, err := sql.Open("sqlite3", db_url)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fmt.Println("Starting Parser")
	client := &http.Client{}
	req, err := http.NewRequest("GET", "https://api1.raildata.org.uk/1010-knowlegebase-toc-xml-feed2_0/4.0/tocs.xml", nil)
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	req.Header.Set("x-apikey", toc_api_key)
	res, err := client.Do(req)
	if err != nil {
		fmt.Println(err)
		os.Exit(3)
	}
	byteValue, err := io.ReadAll(res.Body)
	if err != nil {
		fmt.Println(err)
		os.Exit(4)
	}
	var operatingCompanies TrainOperatingCompanyList
	if xml.Unmarshal(byteValue, &operatingCompanies) != nil {
		fmt.Println(err)
		os.Exit(5)
	}

	for i := 0; i < len(operatingCompanies.TrainCompany); i++ {
		var company = operatingCompanies.TrainCompany[i]

		stmt, err := db.Prepare("INSERT INTO train_operator (name, atoc_code, logo, legalName)  VALUES (?1, ?2, ?3, ?4) ON CONFLICT(legalName) DO UPDATE SET name = ?1, atoc_code = ?2, logo = ?3, legalName = ?4")
		if err != nil {
			fmt.Println(err)
			os.Exit(6)
		}
		_, err = stmt.Exec(company.Name, company.AtocCode, company.Logo, company.LegalName)
		if err != nil {
			fmt.Println(err)
			os.Exit(7)
		}

		if company.StationOperator {
			println("ATOC Code: " + company.AtocCode)
			req, err := http.NewRequest("GET", "https://api1.raildata.org.uk/1010-knowlegebase-stations-xml-feed1_1/4.0/stations-"+operatingCompanies.TrainCompany[i].AtocCode+".xml", nil)
			if err != nil {
				fmt.Println(err)
				os.Exit(8)
			}
			req.Header.Set("x-apikey", station_api_key)
			res, err := client.Do(req)
			if err != nil {
				fmt.Println(err)
				os.Exit(9)
			}
			byteValue, err := io.ReadAll(res.Body)
			if err != nil {
				fmt.Println(err)
				os.Exit(10)
			}
			var stationList StationList
			if xml.Unmarshal(byteValue, &stationList) != nil {
				fmt.Println(err)
				os.Exit(11)
			}
			println("Saving " + fmt.Sprintf("%d", len(stationList.Station)) + " Stations")
			for x := 0; x < len(stationList.Station); x++ {
				var station = stationList.Station[x]
				stmt, err := db.Prepare("INSERT INTO train_station (name, longitude, latitude, code, nationalLocationCode) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(nationalLocationCode) DO UPDATE SET name = ?1, longitude = ?2, latitude = ?3, code = ?4, nationalLocationCode = ?5")
				if err != nil {
					fmt.Println(err)
					os.Exit(12)
				}
				_, err = stmt.Exec(station.Name, station.Longitude, station.Latitude, station.Code, station.AlternativeIdentifiers.NationalLocationCode)
				if err != nil {
					fmt.Println(err)
					os.Exit(13)
				}
			}
		} else {
			println("Skipping Operator Code " + operatingCompanies.TrainCompany[i].AtocCode + " due to no stations")
		}
	}
}
