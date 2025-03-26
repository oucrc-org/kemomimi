import React from 'react';
import AddIcon from '@mui/icons-material/Add';

interface AddButtonProps {
  onOpenModal: () => void;
}

const AddButton: React.FC<AddButtonProps> = ({ onOpenModal }) => {
  return (
    <button
      onClick={onOpenModal}
      className="flex items-center border p-2 rounded hover:bg-gray-100 transition-colors"
    >
      <AddIcon className="mr-1" />
      <span>備品追加</span>
    </button>
  );
};

export default AddButton;
