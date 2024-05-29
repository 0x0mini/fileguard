import React, { useState } from 'react';
import axios from 'axios';

const useFileEncryption = () => {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [isBusy, setIsBusy] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const onFileSelectionChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files.length > 0) {
      setErrorMessage(null);
      setSelectedFile(event.target.files[0]);
    } else {
      // Handle case where file selection is cleared
      setSelectedFile(null);
      setErrorMessage('File selection was cleared. Please select a file.');
    }
  };

  const performFileEncryption = async () => {
    if (!selectedFile) {
      setErrorMessage('No file selected for encryption.');
      return;
    }
    setIsBusy(true);
    try {
      const formData = new FormData();
      formData.append('file', selectedFile);
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/encrypt`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
    } catch (err) {
      const message = err.response?.data?.message ?? 'An unexpected error occurred during file encryption.';
      setErrorMessage(message);
    } finally {
      setIsBusy(false);
    }
  };

  const performFileDecryption = async () => {
    if (!selectedFile) {
      setErrorMessage('No file selected for decryption.');
      return;
    }
    setIsBusy(true);
    try {
      const formData = new FormData();
      formData.append('file', selectedFile);
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/decrypt`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      });
    } catch (err) {
      const message = err.response?.data?.message ?? 'An unexpected error occurred during file decryption.';
      setErrorMessage(message);
    } finally {
      setIsBusy(false);
    }
  };

  return {
    onFileSelectionChange,
    performFileEncryption,
    performFileDecryption,
    isBusy,
    errorMessage,
  };
};

export default useFileEncryption;