import { chromium } from 'playwright';
import * as fs from 'fs';
import * as path from 'path';

interface ScrapedData {
  setList: string[];
  musicians: string[];
}

interface ConcertInfo {
  artist: string;
  source: string;
  setList: {
    songNumber: number;
    title: string;
  }[];
  musicians: {
    musicianNumber: number;
    name: string;
    instrument?: string;
  }[];
}

async function scrapeData(url: string): Promise<void> {
  // Launch the browser
  const browser = await chromium.launch();
  const page = await browser.newPage();
  
  try {
    // Navigate to the URL
    console.log(`Navigating to ${url}...`);
    await page.goto(url, { waitUntil: 'domcontentloaded' });
    
    // Extract the artist name from the title
    const title = await page.title();
    const artistName = title.split(':')[0].trim();
    console.log(`Artist: ${artistName}`);
    
    // Wait for the storytext div to be available
    await page.waitForSelector('#storytext', { timeout: 10000 });
    
    // Look for the SET LIST and MUSICIANS sections
    const data = await page.evaluate(() => {
      // Find the storytext container
      const storytext = document.querySelector('#storytext');
      if (!storytext) return { setList: [], musicians: [] };
      
      // Look for the SET LIST and MUSICIANS paragraphs
      const paragraphs = storytext.querySelectorAll('p');
      let setListParagraph = null;
      let musiciansParagraph = null;
      
      for (const p of paragraphs) {
        if (p.textContent?.includes('SET LIST')) {
          setListParagraph = p;
        }
        if (p.textContent?.includes('MUSICIANS')) {
          musiciansParagraph = p;
        }
      }
      
      const result: { setList: string[], musicians: string[] } = { setList: [], musicians: [] };
      
      // Extract set list
      if (setListParagraph) {
        const nextElement = setListParagraph.nextElementSibling;
        if (nextElement && nextElement.tagName === 'UL') {
          const listItems = nextElement.querySelectorAll('li');
          result.setList = Array.from(listItems).map(li => {
            // Remove quotes if they exist and trim whitespace
            let text = li.textContent || '';
            // Remove both double quotes and single quotes from start and end
            text = text.trim().replace(/^["']/, '').replace(/["']$/, '');
            return text.trim();
          });
        }
      }
      
      // Extract musicians
      if (musiciansParagraph) {
        const nextElement = musiciansParagraph.nextElementSibling;
        if (nextElement && nextElement.tagName === 'UL') {
          const listItems = nextElement.querySelectorAll('li');
          result.musicians = Array.from(listItems).map(li => {
            let text = li.textContent || '';
            // Remove both double quotes and single quotes from start and end
            text = text.trim().replace(/^["']/, '').replace(/["']$/, '');
            return text.trim();
          });
        }
      }
      
      return result;
    });
    
    // Create output filename based on artist name
    const sanitizedArtistName = artistName.replace(/[^\w\s]/gi, '').replace(/\s+/g, '_').toLowerCase();
    const outputFileName = `${sanitizedArtistName}_info.json`;
    
    // Log results
    if (data.setList.length > 0) {
      console.log('Set list:');
      data.setList.forEach((song, index) => {
        console.log(`${index + 1}. ${song}`);
      });
    } else {
      console.log('No set list found');
    }
    
    if (data.musicians.length > 0) {
      console.log('\nMusicians:');
      data.musicians.forEach((musician, index) => {
        console.log(`${index + 1}. ${musician}`);
      });
    } else {
      console.log('No musicians list found');
    }
    
    // Parse musicians to separate name and instrument
    const parsedMusicians = data.musicians.map((musician, index) => {
      const parts = musician.split(':');
      if (parts.length === 2) {
        return {
          musicianNumber: index + 1,
          name: parts[0].trim(),
          instrument: parts[1].trim()
        };
      } else {
        return {
          musicianNumber: index + 1,
          name: musician.trim()
        };
      }
    });
    
    // Create JSON structure
    const concertInfo: ConcertInfo = {
      artist: artistName.trim(),
      source: url.trim(),
      setList: data.setList.map((song, index) => ({
        songNumber: index + 1,
        title: song.trim()
      })),
      musicians: parsedMusicians
    };
    
    // Write to file as JSON
    fs.writeFileSync(outputFileName, JSON.stringify(concertInfo, null, 2));
    console.log(`\nInformation saved to ${outputFileName}`);
    
  } catch (error) {
    console.error('Error scraping the data:', error);
  } finally {
    // Close the browser
    await browser.close();
  }
}

// Get URL from command line arguments
const url = process.argv[2];

if (!url) {
  console.error('Please provide a URL as an argument');
  console.log('Usage: npx ts-node scraper.ts <URL>');
  process.exit(1);
}

scrapeData(url)
  .catch(console.error);