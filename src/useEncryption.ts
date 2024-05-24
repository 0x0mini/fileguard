import React, { useState } from 'react';
import axios from 'axios';

const useFileEncrypt = () => {
  const [file, setFile] = useState<File | null>(null);
  const [isProcessing, setIsProcessing] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files[0]) {
      setError(null);
      setFile(event.target.files[0]);
    }
  };

  const encryptFile = async () => {
    if (file) {
      setIsProcessing(true);
      try {
        const formData = new FormData();
        formData.append('file', file);
        const response = await axios.post(`${process.env.REACT_APP_BACKEND_URL}/encrypt`, formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        setIsProcessing(false);
      } catch (err) {
        setIsProcessing(false);
        setError(err.response?.data?.message || 'An error occurred during file encryption.');
      }
    } else {
      setError('No file selected for encryption.');
    }
  };

  const decryptFile = async () => {
    if (file) {
      setIsProcessing(true);
      try {
        const formData = new FormData();
        formData.append('file', file);
        const response = await axios.post(`${process.env.REACT_APP_BACKEND_URL}/decrypt`, formData, {
          headers: {
            'Content-Type': 'multipart/form-data',
          },
        });
        setIsProcessing(false);
      } catch (err) {
        setIsProcessing(false);
        setError(err.response?.data?.message || 'An error occurred during file decryption.');
      }
    } else {
      setError('No file selected for decryption.');
    }
  };

  return {
    handleFileChange,
    encryptFile,
    decryptFile,
    isProcessing,
    error,
  };
};

export default useFileEncrypt;