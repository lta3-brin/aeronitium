import 'package:dart_nats/dart_nats.dart' show Client;
import 'package:dotenv/dotenv.dart' show load, env;
import 'package:socket_io/socket_io.dart' show Server;

void main() {
  load();
  print('Menjalankan Aeronitum adapter...');

  connection();
}

void connection() {
  var io = Server();

  io.on('connect', (client) async {
    var natsaddr = env['NATS_ADDRESS'];
    var nc = Client();

    try {
      await nc.connect(natsaddr);

      var sub = nc.sub('dtc');
      var msg = sub.stream;

      while (msg.isBroadcast) {
        var dtc = await msg.first;

        client.emit('dtc', dtc.string);
      }
    } catch (err) {
      print(err.toString());
      client.emit('dtc', '');
    }
  });

  io.listen(3000);
}
