import open3d as o3d
import numpy as np

def main():
    file_name = "../assets/office1.pcd"
    pcd = o3d.io.read_point_cloud("../assets/office1.pcd")
    o3d.io.write_point_cloud("../assets/new_binary.pcd", pcd, compressed=False, write_ascii=False)

if __name__ == "__main__":
    main()
