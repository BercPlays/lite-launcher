/**
 * @param {Object.<*,*>[]} data
 * @param {String} prop
 * @param {String} value
 * @returns {Object}
 */
const objectArrayPropsSearch = (data, prop, value) => {
	return data.filter((item) => item[prop] === value);
};

export default objectArrayPropsSearch;
