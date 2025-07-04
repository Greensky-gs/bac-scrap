/**
 * @type {"fetch" | 'search' | null}
 */
const param = process.argv.includes('--fetch') ? 'fetch' :
  process.argv.includes('--search') ? 'search' : null

/**
 * @param {{ defaultValue?: string | number; long?: boolean; }} options 
 */
const getNamedParameter = (name, options) => {
    const index = process.argv.indexOf(`--${name}`);

    const def = options?.defaultValue ?? null;
    if (index === -1) return def;

    if (options?.long) {
        const value = process.argv.slice(index + 1).join(' ');
        return value
    }
    return process.argv[index + 1];
}


module.exports.getNamedParameter = getNamedParameter
module.exports.param = param