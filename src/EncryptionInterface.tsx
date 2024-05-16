import React, { useState, useCallback, memo } from 'react';

interface IEncryptionOption {
  value: string;
  label: string;
}

const encryptionOptions: IEncryptionOption[] = [
  { value: 'AES', label: 'AES' },
  { value: 'RSA', label: 'RSA' },
  { value: 'Blowfish', label: 'Blowfish' },
];

const FileEncryptionTool: React.FC = memo(() => {
  const [encryptionMethod, setEncryptionMethod] = useState<string>('');
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [encryptionProgress, setEncryptionProgress] = useState<number>(0);
  const [encryptionError, setEncryptionError] = useState<string>('');

  const handleSelectedFileChange = useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setSelectedFile(event.target.files[0]);
    }
  }, []);

  const handleEncryptionMethodChange = useCallback((event: React.ChangeEvent<HTMLSelectElement>) => {
    setEncryptionMethod(event.target.value);
  }, []);

  const initiateEncryptionProcess = useCallback(async () => {
    if (!selectedFile || !encryptionMethod) {
      setEncryptionError('Both file and encryption method must be selected.');
      return;
    }

    setEncryptionError('');
    setEncryptionProgress(0);

    try {
      for (let i = 0; i <= 100; i += 10) {
        // Using functional update to ensure we have the latest state
        setEncryptionProgress((prevProgress) => prevProgress + 10);
        await new Promise((resolve) => setTimeout(resolve, 100)); // Simulating encryption progress
      }

      alert('File encrypted successfully.');
    } catch (error: unknown) {
      const errorMsg = error instanceof Error ? error.message : 'An unknown error occurred during encryption.';
      setEncryptionError(errorMsg);
    }
  }, [selectedFile, encryptionMethod]);

  return (
    <div>
      <h2>Encrypt Your File</h2>
      <label htmlFor="file-upload">
        Upload a file:
        <input type="file" id="file-upload" onChange={handleSelectedFileChange} />
      </label>
      <br />
      <label htmlFor="encryption-method">
        Select Encryption Method:
        <select id="encryption-method" value={encryptionMethod} onChange={handleEncryptionMethodChange}>
          <option value="">Select an option</option>
          {encryptionOptions.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
      <br />
      <button onClick={initiateEncryptionProcess}>Start Encryption</button>
      {encryptionProgress > 0 && <div>Encryption Progress: {encryptionProgress}%</div>}
      {encryptionError && <div style={{ color: 'red' }}>Error: {encryptionError}</div>}
    </div>
  );
});

export default FileEncryptionTool;