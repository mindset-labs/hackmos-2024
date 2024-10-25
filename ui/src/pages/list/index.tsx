"use client";

import ListPropertyForm from "@/components/ListProperty";
import Navbar from "@/components/Navbar";

export default function List() {
  return (
    <div className="bg-white">
      <Navbar />
      <div className="mt-14">
        <ListPropertyForm />
      </div>
    </div>
  );
}
