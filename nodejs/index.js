const crypto = require('crypto');
const fs = require('fs');

function generateFileMD5(filePath) {
    return new Promise((resolve, reject) => {
        const hash = crypto.createHash('md5');
        const stream = fs.createReadStream(filePath);

        stream.on('data', (data) => {
            hash.update(data);
        });

        stream.on('end', () => {
            const md5Hash = hash.digest('hex');
            resolve(md5Hash);
        });

        stream.on('error', (err) => {
            reject(err);
        });
    });
}

const filePath = '../video.mkv'; // 替换为你的文件路径

const start = Date.now()
generateFileMD5(filePath)
    .then((md5Hash) => {
        console.log("time", Date.now() - start)
        console.log(`MD5 hash of file "${filePath}" is: ${md5Hash}`);
    })
    .catch((err) => {
        console.error('Error calculating MD5 hash:', err);
    });
