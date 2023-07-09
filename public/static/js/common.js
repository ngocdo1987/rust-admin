var validator = {
    string_length: function(value, min, max) { 
        return function(value) { 
            if (value.length < min && value.length > max) { 
                if (arguments.length >= 4 && arguments[4] === true) { 
                    return 'The length must be' + min + ' - ' + max + 'between';
                }
            }
        };
    }
};
