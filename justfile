download url:
	./download.sh "{{url}}"

scrape url:
	cargo run --bin scraper "$URL"

archive year month:
	npx ts-node archive-scraper.ts {{year}} {{month}}
