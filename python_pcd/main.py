import open3d as o3d
import numpy as np

def main():
    pcd = o3d.io.read_point_cloud("../assets/office1.pcd")
    o3d.io.write_point_cloud("hak_binary2.pcd", pcd, compressed=False, write_ascii=False)

if __name__ == "__main__":
    main()
