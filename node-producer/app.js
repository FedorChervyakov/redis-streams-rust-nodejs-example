import { createClient } from 'redis';

(async () =>  {
    const client = createClient({
        url: 'redis://default:averystrongpassword@redis:6379'
    });

    client.on('error', (err) => console.log('Redis Client Error', err));

    await client.connect();

    await client.xAdd("mystream", "*", {
        name: "Sara",
        surname: "OConnor",
    });
    await client.xAdd("mystream", "*", {
        name: "Fedor",
        surname: "Chervyakov",
    });

    const l = await client.xLen("mystream");

    console.log(l);

    await client.quit();
})();
