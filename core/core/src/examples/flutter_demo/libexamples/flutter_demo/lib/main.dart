import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

void main() => runApp(const EddyThinkDemo());

class EddyThinkDemo extends StatefulWidget {
  const EddyThinkDemo({super.key});
  @override State<EddyThinkDemo> createState() => _EddyThinkDemoState();
}

class _EddyThinkDemoState extends State<EddyThinkDemo> {
  final TextEditingController _controller = TextEditingController();
  String _response = "Ask me anything...";
  String _token = "";
  int _credits = 0;
  bool _loading = false;

  final String routstrUrl = "http://10.0.2.2:8000/v1/chat/completions"; // Android emulator

  @override
  void initState() {
    super.initState();
    _loadToken();
  }

  Future<void> _loadToken() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _token = prefs.getString('eddythink_token') ?? '';
      _credits = prefs.getInt('eddythink_credits') ?? 0;
    });
  }

  Future<void> _saveToken(String token, int credits) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('eddythink_token', token);
    await prefs.setInt('eddythink_credits', credits);
  }

  Future<void> _sendPrompt() async {
    if (_token.isEmpty) {
      setState(() => _response = "No credits! Get a token from eddy.cash/think");
      return;
    }

    setState(() => _loading = true);

    try {
      final response = await http.post(
        Uri.parse(routstrUrl),
        headers: {"Content-Type": "application/json", "x-cashu": _token},
        body: jsonEncode({
          "model": "llama3-70b-8192",
          "messages": [{"role": "user", "content": _controller.text}],
        }),
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        setState(() {
          _response = data['choices'][0]['message']['content'];
          _credits -= 1;
          _saveToken(_token, _credits);
        });
      } else if (response.statusCode == 402) {
        setState(() => _response = "Out of credits! Upgrade at eddy.cash/think");
      } else {
        setState(() => _response = "Error: ${response.statusCode}");
      }
    } catch (e) {
      setState(() => _response = "Network error: $e");
    }

    setState(() => _loading = false);
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'EddyThink v2 Demo',
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        appBar: AppBar(
          backgroundColor: const Color(0xFFF7931A),
          title: const Text('EddyThink v2', style: TextStyle(color: Colors.white)),
          actions: [
            Padding(
              padding: const EdgeInsets.all(16),
              child: Center(child: Text("Credits: $_credits", style: const TextStyle(fontSize: 18, color: Colors.white))),
            )
          ],
        ),
        body: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            children: [
              TextField(
                controller: _controller,
                decoration: const InputDecoration(labelText: "Ask the AI anything...", border: OutlineInputBorder()),
                maxLines: 3,
              ),
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: _loading ? null : _sendPrompt,
                child: _loading ? const CircularProgressIndicator() : const Text("Send"),
              ),
              const SizedBox(height: 30),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(color: Colors.grey[100], borderRadius: BorderRadius.circular(12)),
                  child: SingleChildScrollView(child: Text(_response)),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
