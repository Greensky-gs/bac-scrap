const colorate = (text, color) => {
    return `\x1b[${color}m${text}\x1b[0m`;
}

module.exports.colorate = colorate;