require('./scripts/init')
const { writeFileSync, write } = require('node:fs')
const { get } = require('axios')
const { param, getnamedParameter } = require('./scripts/param')
const metadata = require('../metadata.json');
const { logger } = require('./scripts/logger');
const results = require('../results.json')
const { colorate }  = require('./scripts/color');

const url = (name) => `https://resultats.examens-concours.gouv.fr/api/BGT/publication?filtre=${name}&contexte=QkdULEExNiwyMDI1OkE6QkdULTIuMywxLCwsLA==`;

if (!param) {
    class ParameterError extends Error {
        constructor() {
            super('You must specify a parameter: --fetch or --search');
        }
    }
    throw new ParameterError()
} 

if (param === 'fetch') {
    const main = async() => {
        const alphabet = "abcdefghijklmnopqrstuvwxyz";
        const [sa, sb, sc, sd] = metadata.indexes

        const fetch = async(name) => {
            logger('neutral', 'Fetching', `Fetching ${colorate(name, 33)}...`);

            const nameUrl = url(name);
            const res = await get(nameUrl).catch(() => {});

            if (!res || res?.status !== 200) {
                logger('unexpected', 'Fetch Error', `Failed to fetch ${colorate(name, 33)}: ${res.status}. Writing`);

                metadata.errors.push(name);
                writeFileSync('./metadata.json', JSON.stringify(metadata));
                return
            }

            
            /**
             * @type {{ nom: string; prenoms: string; resultat: string; homonyme: boolean; }[]}
             */
            const resultat = res.data.results 
            if (!resultat) return logger('neutral', 'Fetch Result', `No result found for ${colorate(name, 33)}. Skipping.`);
            logger('expected', 'Fetch Success', `Fetched ${colorate(name, 33)} successfully. Result: ${resultat.resultat}`);

            results.push(...resultat)
        }

    
        for (let a = sa; a < alphabet.length; a++) {
            await fetch(alphabet[a]);
            for (let b = sb; b < alphabet.length; b++) {
                await fetch(`${alphabet[a]}${alphabet[b]}`)
            
                for (let c = sc; c < alphabet.length; c++) {
                    await fetch(`${alphabet[a]}${alphabet[b]}${alphabet[c]}`);
                
                    for (let d = sd; d < alphabet.length; d++) {
                        const name = `${alphabet[a]}${alphabet[b]}${alphabet[c]}${alphabet[d]}`;
                        await fetch(name);
                    }
                }
            
                logger('good', 'Fetch Progress', `Fetched successfully. Moving to next letter. Writing.`);
                metadata.indexes = [a, b, sc, sd]
                writeFileSync('./metadata.json', JSON.stringify(metadata));
                writeFileSync('./results.json', JSON.stringify(results));
            }
        }

        logger('good', 'Fetch Complete', `All names fetched successfully. Writing results.`);
        writeFileSync('./results.json', JSON.stringify(results));
        logger('brilliant', 'Fetch Complete', `All names fetched successfully. Results written to results.json`);
    }

    main()
}