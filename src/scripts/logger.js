/**
 * 
 * @param {'blunder' | 'error' | 'unexpected' | 'neutral' | 'expected' | 'good' | 'brilliant'} type 
 * @param {string} title 
 * @param {string} msg 
 */
const logger = (type, title, msg) => {
    const colors = {
        blunder: '\x1b[31m',
        error: '\x1b[91m',
        unexpected: '\x1b[33m',
        neutral: '\x1b[35m',
        expected: '\x1b[92m',
        good: '\x1b[32m',
        brilliant: '\x1b[36m'
    };

    const maxLength = 20;
    const color = colors[type] ?? '\x1b[0m';

    const formattedTitle = title.length > maxLength ? title.slice(0, maxLength - 3) + '...' : title.padEnd(maxLength, ' ');

    console.log(`${color}${formattedTitle}\x1b[0m: ${msg}`);
}

module.exports.logger = logger