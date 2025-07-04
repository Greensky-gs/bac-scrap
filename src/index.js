require('./scripts/init')
const { writeFileSync, write } = require('node:fs')
const { get } = require('axios')
const { param, getNamedParameter } = require('./scripts/param')
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
                logger('unexpected', 'Fetch Error', `Failed to fetch ${colorate(name, 33)}: ${res?.status ?? 'nothing'}. Writing`);

                metadata.errors.push(name);
                writeFileSync('./metadata.json', JSON.stringify(metadata));
                return
            }

            
            /**
             * @type {{ nom: string; prenoms: string; resultat: string; homonyme: boolean; }[]}
             */
            const resultat = res.data.results 
            if (!resultat || !resultat.length) return logger('neutral', 'Fetch Result', `No result found for ${colorate(name, 33)}. Skipping.`);
            logger('expected', 'Fetch Success', `Fetched ${colorate(name, 33)} successfully. Result: ${resultat.length} results.`);

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
if (param === 'search') {
    const name = getNamedParameter('name');
    /**
     * @type {{ nom: string; prenoms: string; resultat: string; homonyme: boolean; }[]}
     */
    const matches = results.filter((result) => result.nom.toLowerCase().includes(name.toLowerCase()) || name.toLowerCase().includes(result.nom.toLowerCase()) || result.prenoms.toLowerCase().includes(name.toLowerCase()) || name.toLowerCase().includes(result.prenoms.toLowerCase()));

    if (!matches.length) {
        logger('neutral', 'Search Result', `No results found for ${colorate(name, 33)}.`);
        return;
    }

    const headers = ['nom', 'prénoms', 'résultat']
    const maxLengthName = Math.max(...matches.concat([{ nom: headers[0] }]).map((match) => match.nom.length));
    const maxLengthPrenoms = Math.max(...matches.concat([{ prenoms: headers[1] }]).map((match) => match.prenoms.length));
    const maxLengthResult = Math.max(...matches.concat([{ resultat: headers[2] }]).map((match) => match.resultat.length));

    const lengthNameTitle = maxLengthName + 2;
    const lengthPrenomsTitle = maxLengthPrenoms + 2;
    const lengthResultTitle = maxLengthResult + 2;

    const header = `╔${'═'.repeat(lengthNameTitle)}╦${'═'.repeat(lengthPrenomsTitle)}╦${'═'.repeat(lengthResultTitle)}╗`;
    const separator = `╠${'═'.repeat(lengthNameTitle)}╬${'═'.repeat(lengthPrenomsTitle)}╬${'═'.repeat(lengthResultTitle)}╣`;
    const footer = header.replace('╔', '╚').replace('╗', '╝').replace(/╦/g, '╩');

    const content = matches.map(m => `║ ${m.nom.padEnd(maxLengthName)} ║ ${m.prenoms.padEnd(maxLengthPrenoms)} ║ ${m.resultat.padEnd(maxLengthResult)} ║`).join('\n');

    console.log(
        header + '\n' +
        `║ ${headers[0].padEnd(maxLengthName)} ║ ${headers[1].padEnd(maxLengthPrenoms)} ║ ${headers[2].padEnd(maxLengthResult)} ║\n` +
        separator + '\n' +
        content + '\n' +
        footer
    );
}