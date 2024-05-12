import React, { useState } from 'react';

interface EncryptionStandard {
  value: string;
  label: string;
}

const encryptionStandards: EncryptionStandard[] = [
  { value: 'AES', label: 'AES' },
  { value: 'RSA', label: 'RSA' },
  { value: 'Blowfish', label: 'Blowfish' },
];

const EncryptionInterface: React.FC = () => {
  const [selectedEncryption, setSelectedEncryption] = useState<string>('');
  const [file, setFile] = useState<File | null>(null);
  const [progress, setProgress] = useState<number>(0);
  const [error, setError] = useState<string>('');

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleEncryptionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedEncryption(event.target.value);
  };

  const startEncryption = async () => {
    if (!file || !selectedEncryption) {
      setError('File and encryption standard must be selected');
      return;
    }

    setError('');
    setProgress(0);

    try {
      for (let i = 0; i <= 100; i += 10) {
        setProgress(i);
        await new Promise((resolve) => setTimeout(resolve, 100)); // Mock progress
      }

      alert('File encrypted successfully');
    } catch (e: any) {
      setError(e.message || 'An error occurred during encryption');
    }
  };

  return (
    <div>
      <h2>Encrypt Your File</h2>
      <label htmlFor="file-upload">
        Upload a file:
        <input type="file" id="file-upload" onChange={handleFileChange} />
      </label>
      <br />
      <label htmlFor="encryption-standard">
        Select Encryption Standard:
        <select id="encryption-standard" value={selectedEncryption} onChange={handleEncryptionChange}>
          <option value="">Select an option</option>
          {encryptionStandards.map((standard) => (
            <option key={standard.value} value={standard.value}>
              {standard.label}
            </option>
          ))}
        </select>
      </label>
      <br />
      <button onClick={startEncryption}>Start Encryption</button>
      {progress > 0 && <div>Encryption Progress: {progress}%</div>}
      {error && <div style={{ color: 'red' }}>Error: {error}</div>}
    </div>
  );
};

export default EncryptionInterface;