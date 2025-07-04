const { existsSync, writeFileSync } = require('node:fs')
const { logger } = require('./logger')

logger('neutral', 'init', 'Initializing configs...')
if (!existsSync('metadata.json')) {
    logger('neutral', 'init', 'Creating metadata.json file')

    writeFileSync('metadata.json', JSON.stringify({
        errors: [],
        indexes: [0, 0, 0, 0]
    }))
}
if (!existsSync('results.json')) {
    logger('neutral', 'init', 'Creating results.json file')

    writeFileSync('results.json', JSON.stringify([]))
}
logger('expected', 'init', 'Initialization complete')