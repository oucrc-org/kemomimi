import React from 'react';
import Button from '@mui/material/Button';
import AddIcon from '@mui/icons-material/Add';

interface AddButtonProps {
  onOpenModal: () => void;
}

const AddButton: React.FC<AddButtonProps> = ({ onOpenModal }) => {
  return (
    <Button
      variant="contained"
      color="primary"
      startIcon={<AddIcon />}
      onClick={onOpenModal}
    >
      備品追加
    </Button>
  );
};

export default AddButton;
