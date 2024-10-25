import { AnyCnameRecord } from "dns";
import React, { useState } from "react";

const CreateTrustForm = () => {
  const [propertyData, setPropertyData] = useState({
    image: null,
    category: "",
    name: "",
    description: "",
    symbol: "",
  });

  const handleChange = (e: any) => {
    const { name, value, files } = e.target;
    setPropertyData({
      ...propertyData,
      [name]: files ? files[0] : value, // handle file upload for image
    });
  };

  const handleSubmit = (e: AnyCnameRecord) => {
    e.preventDefault();
    // Here you can add your submit logic, like sending data to your backend
    console.log("Property Data:", propertyData);
  };

  return (
    <form className="max-w-2xl mx-auto p-6">
      <p className="mt-10 text-2xl text-center font-bold mb-6">
        Create a Trust
      </p>
      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Trust Logo
        </label>
        <div className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-lg p-6">
          <input
            type="file"
            name="image"
            accept="image/*"
            onChange={handleChange}
            className="hidden"
          />
          <button
            type="button"
            className="bg-blue-500 text-white py-2 px-4 rounded-md"
            onClick={() => document.querySelector('input[type="file"]').click()}
          >
            Select Image
          </button>
          <p className="text-gray-500 text-sm mt-2">
            Upload an image of the property (max 5MB, .jpg, .png, .webp)
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6 mb-6">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Name
          </label>
          <input
            type="text"
            name="monthlyIncome"
            value={propertyData.name}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="Dubai REIT"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The legal name of the trust.
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">
            Symbol
          </label>
          <input
            type="number"
            name="pricePerShare"
            value={propertyData.symbol}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="MYTOKEN"
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            The symbol of the governing token of the DAO.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-1 gap-6 mb-6">
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Description
          </label>
          <input
            type="text"
            name="numberOfShares"
            value={propertyData.description}
            onChange={handleChange}
            className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            placeholder="..."
            required
          />
          <p className="text-xs text-gray-500 mt-1">
            Description of the Trust.
          </p>
        </div>
      </div>

      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700">
          Category
        </label>
        <select
          name="category"
          value={propertyData.category}
          onChange={handleChange}
          className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          required
        >
          <option value="">Select a category</option>
          <option value="Residential">Residential</option>
          <option value="Commercial">Commercial</option>
        </select>
        <p className="text-xs text-gray-500 mt-1">
          Select the category that best describes this property.
        </p>
      </div>

      <button
        onClick={handleSubmit}
        className="w-full bg-black text-white py-3 rounded-md font-medium hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-black"
      >
        Create Trust
      </button>
    </form>
  );
};

export default CreateTrustForm;
