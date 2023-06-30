const fs = require('fs');

fs.readFile('./build/mp.chan', 'utf8', (err, data) => {
    let point = 0;

    while (data[point] !== "&") {
        point++;
    }

    let match = data.substring(0, point);
    let message = data.substring(point);

    if (err) {
        return;
    }

    let out;

    if (match) {
        let exp = match == "<temp>" ? new RegExp("<temp> *(\n?[\\S| ]?\n*)*<temp/>") :
            new RegExp(`${match} *{(\n?[\\S| ]?\n*)*}`, 'i');

        let mat = message.match(exp)

        out = `${message.search(exp)}&${mat ? mat[0]: null}`
    } else {
        let regs = [
            /dom *{(\n?[\S| ]?\n*)*}/gi,
            /cam *{(\n?[\S| ]?\n*)*}/gi,
            /sin *{(\n?[\S| ]?\n*)*}/gi,
        ]

        let section_id = 0;
        let arr = [[], [], []];

        for (let exp of regs) {

            let curr = exp.exec(message);

            while (curr != null) {
                let idx = curr.index;

                arr[section_id].push([idx, curr[0].length + idx])
                curr = exp.exec(message)
            }

            section_id++
        }

        for (let $arr of arr) {
            for (let [a, b] of $arr) {
                out += `${a}$${b}\n`
            }
            out += "#\n"
        }
}

    let emp = new Function();

    fs.writeFile("./build/mp.chan", "\n" + out, emp)
});