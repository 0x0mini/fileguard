import React, { useState } from 'react';
import axios from 'axios';

const useFileEncryption = () => {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [isBusy, setIsBusy] = useState<boolean>(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const onFileSelectionChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files[0]) {
      setErrorMessage(null);
      setSelectedFile(event.target.files[0]);
    }
  };

  const performFileEncryption = async () => {
    if (selectedFile) {
      setIsBusy(true);
      try {
        const formData = new FormData();
        formData.append('file', selectedFile);
        await axios.post(`${process.env.REACT_APP_BACKEND_URL}/encrypt`, formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        setIsBusy(false);
      } catch (err) {
        setIsBusy(false);
        setErrorMessage(err.response?.data?.message || 'An error occurred during file encryption.');
      }
    } else {
      setErrorMessage('No file selected for encryption.');
    }
  };

  const performFileDecryption = async () => {
    if (selectedFile) {
      setIsBusy(true);
      try {
        const formData = new FormData();
        formData.append('file', selectedFile);
        await axios.post(`${process.env.REACT_APP_BACKEND_URL}/decrypt`, formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        setIsBusy(false);
      } catch (err) {
        setIsBusy(false);
        setErrorMessage(err.response?.data?.message || 'An error occurred during file decryption.');
      }
    } else {
      setErrorMessage('No file selected for decryption.');
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