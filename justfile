download url:
	./download.sh "{{url}}"

scrape url:
	npx ts-node scraper.ts "{{url}}"

archive year month:
	npx ts-node archive-scraper.ts {{year}} {{month}}
