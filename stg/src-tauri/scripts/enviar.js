import pkg from 'whatsapp-web.js';
import qrcode from 'qrcode-terminal';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Fix __dirname issue in ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Destructure the CommonJS module
const { Client, LocalAuth } = pkg;

// Initialize the WhatsApp client
const client = new Client({
  authStrategy: new LocalAuth({ dataPath: path.join(__dirname, '../session') }),
  puppeteer: { 
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  }
});

// Function to validate and format phone numbers
const validatePhoneNumber = (number) => {
  const cleaned = number.replace(/[^\d]/g, ''); // Remove non-numeric characters
  return cleaned.length >= 10 ? `${cleaned}` : null; // Add Colombian country code (57)
};

// Generate QR code for authentication
client.on('qr', qr => {
  qrcode.generate(qr, { small: true });
  fs.writeFileSync(path.join(__dirname, '../qrcode.txt'), qr);
  console.log('QR code generated. Scan it with your phone.');
});

// Handle successful authentication
client.on('authenticated', () => {
  console.log('Authenticated!');
  fs.unlinkSync(path.join(__dirname, '../qrcode.txt')); // Delete QR code file
});

// Handle authentication failure
client.on('auth_failure', (msg) => {
  console.error('Authentication failed:', msg);
  process.exit(1); // Exit with error code
});

// Handle client disconnection
client.on('disconnected', (reason) => {
  console.error('Client disconnected:', reason);
  process.exit(1); // Exit with error code
});

// Handle client ready event
client.on('ready', async () => {
  console.log('Client is ready!');

  try {
    // Hardcoded test numbers (replace with your actual numbers)
    const testNumbers = [
      '573193388778', // Your Colombian phone number
    ];

    // Hardcoded test message
    const testMessage = "¡Hola! Esto es una prueba de envío de mensajes desde wwebjs.";

    // Send messages to each number
    for (const number of testNumbers) {
      const validNumber = validatePhoneNumber(number);
      if (!validNumber) {
        console.error(`Invalid phone number: ${number}`);
        continue;
      }

      const chatId = `${validNumber}@c.us`;
      console.log(`Attempting to send message to ${chatId}`);

      const exists = await client.getNumberId(chatId);
      if (exists) {
        console.log(`Number ${chatId} is registered on WhatsApp`);
        await client.sendMessage(chatId, testMessage);
        console.log(`Message sent to ${chatId}`);
      } else {
        console.error(`Number ${chatId} is NOT registered on WhatsApp`);
      }
    }
  } catch (error) {
    console.error('Error:', error);
  } finally {
    client.destroy(); // Destroy the client after sending messages
  }
});

// Initialize the client
client.initialize();